mkmod!{constraints, { 
                getname!(constraints);
                getsrc!(constraints);
                getpath!(constraints);
                get_deps!(constraints);
                get_crates!(constraints);
                mkinclude!(constraints);
                 
            }}
mkmod!{dump, { 
                getname!(dump);
                getsrc!(dump);
                getpath!(dump);
                get_deps!(dump);
                get_crates!(dump);
                mkinclude!(dump);
                 
            }}
mkmod!{legacy, { 
                getname!(legacy);
                getsrc!(legacy);
                getpath!(legacy);
                get_deps!(legacy);
                get_crates!(legacy);
                mkinclude!(legacy);
                 
            }}
mkmod!{liveness_constraints, { 
                getname!(liveness_constraints);
                getsrc!(liveness_constraints);
                getpath!(liveness_constraints);
                get_deps!(liveness_constraints);
                get_crates!(liveness_constraints);
                mkinclude!(liveness_constraints);
                 
            }}
mkmod!{loan_liveness, { 
                getname!(loan_liveness);
                getsrc!(loan_liveness);
                getpath!(loan_liveness);
                get_deps!(loan_liveness);
                get_crates!(loan_liveness);
                mkinclude!(loan_liveness);
                 
            }}
mkmod!{typeck_constraints, { 
                getname!(typeck_constraints);
                getsrc!(typeck_constraints);
                getpath!(typeck_constraints);
                get_deps!(typeck_constraints);
                get_crates!(typeck_constraints);
                mkinclude!(typeck_constraints);
                 
            }}
mkuse!{use std :: collections :: BTreeMap ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_index :: bit_set :: SparseBitMatrix ;}
mkuse!{use rustc_index :: interval :: SparseIntervalMatrix ;}
mkuse!{use rustc_middle :: mir :: { Body , Local } ;}
mkuse!{use rustc_middle :: ty :: { RegionVid , TyCtxt } ;}
mkuse!{use rustc_mir_dataflow :: points :: PointIndex ;}
mkuse!{pub (crate) use self :: constraints :: * ;}
mkuse!{pub (crate) use self :: dump :: dump_polonius_mir ;}
mkuse!{use self :: liveness_constraints :: create_liveness_constraints ;}
mkuse!{use self :: loan_liveness :: compute_loan_liveness ;}
mkuse!{use self :: typeck_constraints :: convert_typeck_constraints ;}
mkuse!{use crate :: dataflow :: BorrowIndex ;}
mkuse!{use crate :: { BorrowSet , RegionInferenceContext } ;}
mkitem!{pub (crate) type LiveLoans = SparseBitMatrix < PointIndex , BorrowIndex > ;}
mkitem!{mkstruct!{# [doc = " This struct holds the liveness data created during MIR typeck, and which will be used later in"] # [doc = " the process, to compute the polonius localized constraints."] # [derive (Default)] pub (crate) struct PoloniusLivenessContext { # [doc = " The expected edge direction per live region: the kind of directed edge we'll create as"] # [doc = " liveness constraints depends on the variance of types with respect to each contained region."] live_region_variances : BTreeMap < RegionVid , ConstraintDirection > , # [doc = " The regions that outlive free regions are used to distinguish relevant live locals from"] # [doc = " boring locals. A boring local is one whose type contains only such regions. Polonius"] # [doc = " currently has more boring locals than NLLs so we record the latter to use in errors and"] # [doc = " diagnostics, to focus on the locals we consider relevant and match NLL diagnostics."] pub (crate) boring_nll_locals : FxHashSet < Local > , }}}
mkitem!{mkstruct!{# [doc = " This struct holds the data needed to create the Polonius localized constraints. Its data is"] # [doc = " transferred and converted from the [PoloniusLivenessContext] at the end of MIR typeck."] pub (crate) struct PoloniusContext { # [doc = " The liveness data we recorded during MIR typeck."] liveness_context : PoloniusLivenessContext , # [doc = " The set of regions that are live at a given point in the CFG, used to create localized"] # [doc = " outlives constraints between regions that are live at connected points in the CFG."] live_regions : SparseBitMatrix < PointIndex , RegionVid > , }}}
mkitem!{mkstruct!{# [doc = " This struct holds the data needed by the borrowck error computation and diagnostics. Its data is"] # [doc = " computed from the [PoloniusContext] when computing NLL regions."] pub (crate) struct PoloniusDiagnosticsContext { # [doc = " The localized outlives constraints that were computed in the main analysis."] localized_outlives_constraints : LocalizedOutlivesConstraintSet , # [doc = " The liveness data computed during MIR typeck: [PoloniusLivenessContext::boring_nll_locals]."] pub (crate) boring_nll_locals : FxHashSet < Local > , }}}
mkitem!{mkenum!{# [doc = " The direction a constraint can flow into. Used to create liveness constraints according to"] # [doc = " variance."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] enum ConstraintDirection { # [doc = " For covariant cases, we add a forward edge `O at P1 -> O at P2`."] Forward , # [doc = " For contravariant cases, we add a backward edge `O at P2 -> O at P1`"] Backward , # [doc = " For invariant cases, we add both the forward and backward edges `O at P1 <-> O at P2`."] Bidirectional , }}}
mkitem!{mkimpl!{impl PoloniusContext { # [doc = " Unlike NLLs, in polonius we traverse the cfg to look for regions live across an edge, so we"] # [doc = " need to transpose the \"points where each region is live\" matrix to a \"live regions per point\""] # [doc = " matrix."] pub (crate) fn create_from_liveness (liveness_context : PoloniusLivenessContext , num_regions : usize , points_per_live_region : & SparseIntervalMatrix < RegionVid , PointIndex > ,) -> PoloniusContext { let mut live_regions_per_point = SparseBitMatrix :: new (num_regions) ; for region in points_per_live_region . rows () { for point in points_per_live_region . row (region) . unwrap () . iter () { live_regions_per_point . insert (point , region) ; } } PoloniusContext { live_regions : live_regions_per_point , liveness_context } } # [doc = " Computes live loans using the set of loans model for `-Zpolonius=next`."] # [doc = ""] # [doc = " First, creates a constraint graph combining regions and CFG points, by:"] # [doc = " - converting NLL typeck constraints to be localized"] # [doc = " - encoding liveness constraints"] # [doc = ""] # [doc = " Then, this graph is traversed, reachability is recorded as loan liveness, to be used by the"] # [doc = " loan scope and active loans computations."] # [doc = ""] # [doc = " The constraint data will be used to compute errors and diagnostics."] pub (crate) fn compute_loan_liveness < 'tcx > (self , tcx : TyCtxt < 'tcx > , regioncx : & mut RegionInferenceContext < 'tcx > , body : & Body < 'tcx > , borrow_set : & BorrowSet < 'tcx > ,) -> PoloniusDiagnosticsContext { let PoloniusLivenessContext { live_region_variances , boring_nll_locals } = self . liveness_context ; let mut localized_outlives_constraints = LocalizedOutlivesConstraintSet :: default () ; convert_typeck_constraints (tcx , body , regioncx . liveness_constraints () , regioncx . outlives_constraints () , regioncx . universal_regions () , & mut localized_outlives_constraints ,) ; create_liveness_constraints (body , regioncx . liveness_constraints () , & self . live_regions , & live_region_variances , regioncx . universal_regions () , & mut localized_outlives_constraints ,) ; let live_loans = compute_loan_liveness (regioncx . liveness_constraints () , regioncx . outlives_constraints () , borrow_set , & localized_outlives_constraints ,) ; regioncx . record_live_loans (live_loans) ; PoloniusDiagnosticsContext { localized_outlives_constraints , boring_nll_locals } } }}}
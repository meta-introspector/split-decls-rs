/* FP:dropck_outlives.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_dropck_outlives_USE_0001
/* FP:dropck_outlives.rs-0002 */ use crate :: rustc_data_structures :: fx :: FxHashSet ;
/* FP:dropck_outlives.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_dropck_outlives_USE_0002
/* FP:dropck_outlives.rs-0004 */ use crate :: rustc_infer :: infer :: TyCtxtInferExt ;
/* FP:dropck_outlives.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_dropck_outlives_USE_0003
/* FP:dropck_outlives.rs-0006 */ use crate :: rustc_infer :: infer :: canonical :: { Canonical , QueryResponse } ;
/* FP:dropck_outlives.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_dropck_outlives_USE_0004
/* FP:dropck_outlives.rs-0008 */ use crate :: rustc_complete :: bug ;
/* FP:dropck_outlives.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_dropck_outlives_USE_0005
/* FP:dropck_outlives.rs-0010 */ use crate :: rustc_complete :: query :: Providers ;
/* FP:dropck_outlives.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_dropck_outlives_USE_0006
/* FP:dropck_outlives.rs-0012 */ use crate :: rustc_complete :: traits :: query :: { DropckConstraint , DropckOutlivesResult } ;
/* FP:dropck_outlives.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_dropck_outlives_USE_0007
/* FP:dropck_outlives.rs-0014 */ use crate :: rustc_complete :: ty :: { self , GenericArgs , TyCtxt } ;
/* FP:dropck_outlives.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_dropck_outlives_USE_0008
/* FP:dropck_outlives.rs-0016 */ use crate :: rustc_complete :: DUMMY_SP ;
/* FP:dropck_outlives.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_dropck_outlives_USE_0009
/* FP:dropck_outlives.rs-0018 */ use crate :: rustc_complete :: def_id :: DefId ;
/* FP:dropck_outlives.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_dropck_outlives_USE_0010
/* FP:dropck_outlives.rs-0020 */ use crate :: rustc_trait_selection :: infer :: InferCtxtBuilderExt ;
/* FP:dropck_outlives.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_dropck_outlives_USE_0011
/* FP:dropck_outlives.rs-0022 */ use crate :: rustc_trait_selection :: traits :: query :: dropck_outlives :: { compute_dropck_outlives_inner , dtorck_constraint_for_ty_inner , } ;
/* FP:dropck_outlives.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_dropck_outlives_USE_0012
/* FP:dropck_outlives.rs-0024 */ use crate :: rustc_trait_selection :: traits :: query :: { CanonicalDropckOutlivesGoal , NoSolution } ;
/* FP:dropck_outlives.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_dropck_outlives_USE_0013
/* FP:dropck_outlives.rs-0026 */ use tracing :: debug ;
/* FP:dropck_outlives.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_dropck_outlives_FN_0014
/* FP:dropck_outlives.rs-0028 */ pub (crate) fn provide (p : & mut Providers) { * p = Providers { dropck_outlives , adt_dtorck_constraint , .. * p } ; }
/* FP:dropck_outlives.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_dropck_outlives_FN_0015
/* FP:dropck_outlives.rs-0030 */ fn dropck_outlives < 'tcx > (tcx : TyCtxt < 'tcx > , canonical_goal : CanonicalDropckOutlivesGoal < 'tcx > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , DropckOutlivesResult < 'tcx > > > , NoSolution > { debug ! ("dropck_outlives(goal={:#?})" , canonical_goal) ; tcx . infer_ctxt () . enter_canonical_trait_query (& canonical_goal , | ocx , goal | { compute_dropck_outlives_inner (ocx , goal , DUMMY_SP) }) }
/* FP:dropck_outlives.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_dropck_outlives_FN_0016
/* FP:dropck_outlives.rs-0032 */ # [doc = " Calculates the dtorck constraint for a type."] pub (crate) fn adt_dtorck_constraint (tcx : TyCtxt < '_ > , def_id : DefId) -> & DropckConstraint < '_ > { let def = tcx . adt_def (def_id) ; let span = tcx . def_span (def_id) ; let typing_env = ty :: TypingEnv :: non_body_analysis (tcx , def_id) ; debug ! ("dtorck_constraint: {:?}" , def) ; if def . is_manually_drop () { bug ! ("`ManuallyDrop` should have been handled by `trivial_dropck_outlives`") ; } else if def . is_phantom_data () { let args = GenericArgs :: identity_for_item (tcx , def_id) ; assert_eq ! (args . len () , 1) ; let result = DropckConstraint { outlives : vec ! [] , dtorck_types : vec ! [args . type_at (0)] , overflows : vec ! [] , } ; debug ! ("dtorck_constraint: {:?} => {:?}" , def , result) ; return tcx . arena . alloc (result) ; } let mut result = DropckConstraint :: empty () ; for field in def . all_fields () { let fty = tcx . type_of (field . did) . instantiate_identity () ; dtorck_constraint_for_ty_inner (tcx , typing_env , span , 0 , fty , & mut result) ; } result . outlives . extend (tcx . destructor_constraints (def)) ; dedup_dtorck_constraint (& mut result) ; debug ! ("dtorck_constraint: {:?} => {:?}" , def , result) ; tcx . arena . alloc (result) }
/* FP:dropck_outlives.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_dropck_outlives_FN_0017
/* FP:dropck_outlives.rs-0034 */ fn dedup_dtorck_constraint (c : & mut DropckConstraint < '_ >) { let mut outlives = FxHashSet :: default () ; let mut dtorck_types = FxHashSet :: default () ; c . outlives . retain (| & val | outlives . replace (val) . is_none ()) ; c . dtorck_types . retain (| & val | dtorck_types . replace (val) . is_none ()) ; }
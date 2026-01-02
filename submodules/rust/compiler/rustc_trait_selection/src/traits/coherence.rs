mkuse!{use std :: fmt :: Debug ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashSet , FxIndexSet } ;}
mkuse!{use rustc_errors :: { Diag , EmissionGuarantee } ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: { CRATE_DEF_ID , DefId } ;}
mkuse!{use rustc_infer :: infer :: { DefineOpaqueTypes , InferCtxt , TyCtxtInferExt } ;}
mkuse!{use rustc_infer :: traits :: PredicateObligations ;}
mkuse!{use rustc_macros :: { TypeFoldable , TypeVisitable } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: traits :: query :: NoSolution ;}
mkuse!{use rustc_middle :: traits :: solve :: { CandidateSource , Certainty , Goal } ;}
mkuse!{use rustc_middle :: traits :: specialization_graph :: OverlapMode ;}
mkuse!{use rustc_middle :: ty :: fast_reject :: DeepRejectCtxt ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt , TypeSuperVisitable , TypeVisitable , TypeVisitableExt , TypeVisitor , TypingMode , } ;}
mkuse!{pub use rustc_next_trait_solver :: coherence :: * ;}
mkuse!{use rustc_next_trait_solver :: solve :: SolverDelegateEvalExt ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span , sym } ;}
mkuse!{use tracing :: { debug , instrument , warn } ;}
mkuse!{use super :: ObligationCtxt ;}
mkuse!{use crate :: error_reporting :: traits :: suggest_new_overflow_limit ;}
mkuse!{use crate :: infer :: InferOk ;}
mkuse!{use crate :: solve :: inspect :: { InspectGoal , ProofTreeInferCtxtExt , ProofTreeVisitor } ;}
mkuse!{use crate :: solve :: { SolverDelegate , deeply_normalize_for_diagnostics , inspect } ;}
mkuse!{use crate :: traits :: query :: evaluate_obligation :: InferCtxtExt ;}
mkuse!{use crate :: traits :: select :: IntercrateAmbiguityCause ;}
mkuse!{use crate :: traits :: { FulfillmentErrorCode , NormalizeExt , Obligation , ObligationCause , PredicateObligation , SelectionContext , SkipLeakCheck , util , } ;}
mkitem!{mkstruct!{# [doc = " The \"header\" of an impl is everything outside the body: a Self type, a trait"] # [doc = " ref (in the case of a trait impl), and a set of predicates (from the"] # [doc = " bounds / where-clauses)."] # [derive (Clone , Debug , TypeFoldable , TypeVisitable)] pub struct ImplHeader < 'tcx > { pub impl_def_id : DefId , pub impl_args : ty :: GenericArgsRef < 'tcx > , pub self_ty : Ty < 'tcx > , pub trait_ref : Option < ty :: TraitRef < 'tcx > > , pub predicates : Vec < ty :: Predicate < 'tcx > > , }}}
mkitem!{mkstruct!{pub struct OverlapResult < 'tcx > { pub impl_header : ImplHeader < 'tcx > , pub intercrate_ambiguity_causes : FxIndexSet < IntercrateAmbiguityCause < 'tcx > > , # [doc = " `true` if the overlap might've been permitted before the shift"] # [doc = " to universes."] pub involves_placeholder : bool , # [doc = " Used in the new solver to suggest increasing the recursion limit."] pub overflowing_predicates : Vec < ty :: Predicate < 'tcx > > , }}}

macro_rules! add_placeholder_note_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_placeholder_note in module {}", module_path!());
    };
}

mkfn!{
    add_placeholder_note_introspect!();
    pub fn add_placeholder_note < G : EmissionGuarantee > (err : & mut Diag < '_ , G >) { err . note ("this behavior recently changed as a result of a bug fix; \
         see rust-lang/rust#56105 for details" ,) ; }
}

macro_rules! suggest_increasing_recursion_limit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function suggest_increasing_recursion_limit in module {}", module_path!());
    };
}

mkfn!{
    suggest_increasing_recursion_limit_introspect!();
    pub (crate) fn suggest_increasing_recursion_limit < 'tcx , G : EmissionGuarantee > (tcx : TyCtxt < 'tcx > , err : & mut Diag < '_ , G > , overflowing_predicates : & [ty :: Predicate < 'tcx >] ,) { for pred in overflowing_predicates { err . note (format ! ("overflow evaluating the requirement `{}`" , pred)) ; } suggest_new_overflow_limit (tcx , err) ; }
}
mkitem!{mkenum!{# [derive (Debug , Clone , Copy)] enum TrackAmbiguityCauses { Yes , No , }}}
mkitem!{mkimpl!{impl TrackAmbiguityCauses { fn is_yes (self) -> bool { match self { TrackAmbiguityCauses :: Yes => true , TrackAmbiguityCauses :: No => false , } } }}}

macro_rules! overlapping_impls_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function overlapping_impls in module {}", module_path!());
    };
}

mkfn!{
    overlapping_impls_introspect!();
    # [doc = " If there are types that satisfy both impls, returns `Some`"] # [doc = " with a suitably-freshened `ImplHeader` with those types"] # [doc = " instantiated. Otherwise, returns `None`."] # [instrument (skip (tcx , skip_leak_check) , level = "debug")] pub fn overlapping_impls (tcx : TyCtxt < '_ > , impl1_def_id : DefId , impl2_def_id : DefId , skip_leak_check : SkipLeakCheck , overlap_mode : OverlapMode ,) -> Option < OverlapResult < '_ > > { let drcx = DeepRejectCtxt :: relate_infer_infer (tcx) ; let impl1_ref = tcx . impl_trait_ref (impl1_def_id) ; let impl2_ref = tcx . impl_trait_ref (impl2_def_id) ; let may_overlap = match (impl1_ref , impl2_ref) { (Some (a) , Some (b)) => drcx . args_may_unify (a . skip_binder () . args , b . skip_binder () . args) , (None , None) => { let self_ty1 = tcx . type_of (impl1_def_id) . skip_binder () ; let self_ty2 = tcx . type_of (impl2_def_id) . skip_binder () ; drcx . types_may_unify (self_ty1 , self_ty2) } _ => bug ! ("unexpected impls: {impl1_def_id:?} {impl2_def_id:?}") , } ; if ! may_overlap { debug ! ("overlapping_impls: fast_reject early-exit") ; return None ; } if tcx . next_trait_solver_in_coherence () { overlap (tcx , TrackAmbiguityCauses :: Yes , skip_leak_check , impl1_def_id , impl2_def_id , overlap_mode ,) } else { let _overlap_with_bad_diagnostics = overlap (tcx , TrackAmbiguityCauses :: No , skip_leak_check , impl1_def_id , impl2_def_id , overlap_mode ,) ? ; let overlap = overlap (tcx , TrackAmbiguityCauses :: Yes , skip_leak_check , impl1_def_id , impl2_def_id , overlap_mode ,) . unwrap () ; Some (overlap) } }
}

macro_rules! fresh_impl_header_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fresh_impl_header in module {}", module_path!());
    };
}

mkfn!{
    fresh_impl_header_introspect!();
    fn fresh_impl_header < 'tcx > (infcx : & InferCtxt < 'tcx > , impl_def_id : DefId) -> ImplHeader < 'tcx > { let tcx = infcx . tcx ; let impl_args = infcx . fresh_args_for_item (DUMMY_SP , impl_def_id) ; ImplHeader { impl_def_id , impl_args , self_ty : tcx . type_of (impl_def_id) . instantiate (tcx , impl_args) , trait_ref : tcx . impl_trait_ref (impl_def_id) . map (| i | i . instantiate (tcx , impl_args)) , predicates : tcx . predicates_of (impl_def_id) . instantiate (tcx , impl_args) . iter () . map (| (c , _) | c . as_predicate ()) . collect () , } }
}

macro_rules! fresh_impl_header_normalized_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fresh_impl_header_normalized in module {}", module_path!());
    };
}

mkfn!{
    fresh_impl_header_normalized_introspect!();
    fn fresh_impl_header_normalized < 'tcx > (infcx : & InferCtxt < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , impl_def_id : DefId ,) -> ImplHeader < 'tcx > { let header = fresh_impl_header (infcx , impl_def_id) ; let InferOk { value : mut header , obligations } = infcx . at (& ObligationCause :: dummy () , param_env) . normalize (header) ; header . predicates . extend (obligations . into_iter () . map (| o | o . predicate)) ; header }
}

macro_rules! overlap_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function overlap in module {}", module_path!());
    };
}

mkfn!{
    overlap_introspect!();
    # [doc = " Can both impl `a` and impl `b` be satisfied by a common type (including"] # [doc = " where-clauses)? If so, returns an `ImplHeader` that unifies the two impls."] # [instrument (level = "debug" , skip (tcx))] fn overlap < 'tcx > (tcx : TyCtxt < 'tcx > , track_ambiguity_causes : TrackAmbiguityCauses , skip_leak_check : SkipLeakCheck , impl1_def_id : DefId , impl2_def_id : DefId , overlap_mode : OverlapMode ,) -> Option < OverlapResult < 'tcx > > { if overlap_mode . use_negative_impl () { if impl_intersection_has_negative_obligation (tcx , impl1_def_id , impl2_def_id) || impl_intersection_has_negative_obligation (tcx , impl2_def_id , impl1_def_id) { return None ; } } let infcx = tcx . infer_ctxt () . skip_leak_check (skip_leak_check . is_yes ()) . with_next_trait_solver (tcx . next_trait_solver_in_coherence ()) . build (TypingMode :: Coherence) ; let selcx = & mut SelectionContext :: new (& infcx) ; if track_ambiguity_causes . is_yes () { selcx . enable_tracking_intercrate_ambiguity_causes () ; } let param_env = ty :: ParamEnv :: empty () ; let impl1_header = fresh_impl_header_normalized (selcx . infcx , param_env , impl1_def_id) ; let impl2_header = fresh_impl_header_normalized (selcx . infcx , param_env , impl2_def_id) ; let mut obligations = equate_impl_headers (selcx . infcx , param_env , & impl1_header , & impl2_header) ? ; debug ! ("overlap: unification check succeeded") ; obligations . extend ([& impl1_header . predicates , & impl2_header . predicates] . into_iter () . flatten () . map (| & predicate | Obligation :: new (infcx . tcx , ObligationCause :: dummy () , param_env , predicate) ,) ,) ; let mut overflowing_predicates = Vec :: new () ; if overlap_mode . use_implicit_negative () { match impl_intersection_has_impossible_obligation (selcx , & obligations) { IntersectionHasImpossibleObligations :: Yes => return None , IntersectionHasImpossibleObligations :: No { overflowing_predicates : p } => { overflowing_predicates = p } } } if infcx . leak_check (ty :: UniverseIndex :: ROOT , None) . is_err () { debug ! ("overlap: leak check failed") ; return None ; } let intercrate_ambiguity_causes = if ! overlap_mode . use_implicit_negative () { Default :: default () } else if infcx . next_trait_solver () { compute_intercrate_ambiguity_causes (& infcx , & obligations) } else { selcx . take_intercrate_ambiguity_causes () } ; debug ! ("overlap: intercrate_ambiguity_causes={:#?}" , intercrate_ambiguity_causes) ; let involves_placeholder = infcx . inner . borrow_mut () . unwrap_region_constraints () . data () . constraints . iter () . any (| c | c . 0 . involves_placeholders ()) ; let mut impl_header = infcx . resolve_vars_if_possible (impl1_header) ; if infcx . next_trait_solver () { impl_header = deeply_normalize_for_diagnostics (& infcx , param_env , impl_header) ; } Some (OverlapResult { impl_header , intercrate_ambiguity_causes , involves_placeholder , overflowing_predicates , }) }
}

macro_rules! equate_impl_headers_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function equate_impl_headers in module {}", module_path!());
    };
}

mkfn!{
    equate_impl_headers_introspect!();
    # [instrument (level = "debug" , skip (infcx) , ret)] fn equate_impl_headers < 'tcx > (infcx : & InferCtxt < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , impl1 : & ImplHeader < 'tcx > , impl2 : & ImplHeader < 'tcx > ,) -> Option < PredicateObligations < 'tcx > > { let result = match (impl1 . trait_ref , impl2 . trait_ref) { (Some (impl1_ref) , Some (impl2_ref)) => infcx . at (& ObligationCause :: dummy () , param_env) . eq (DefineOpaqueTypes :: Yes , impl1_ref , impl2_ref) , (None , None) => infcx . at (& ObligationCause :: dummy () , param_env) . eq (DefineOpaqueTypes :: Yes , impl1 . self_ty , impl2 . self_ty ,) , _ => bug ! ("equate_impl_headers given mismatched impl kinds") , } ; result . map (| infer_ok | infer_ok . obligations) . ok () }
}
mkitem!{mkenum!{# [doc = " The result of [fn impl_intersection_has_impossible_obligation]."] # [derive (Debug)] enum IntersectionHasImpossibleObligations < 'tcx > { Yes , No { # [doc = " With `-Znext-solver=coherence`, some obligations may"] # [doc = " fail if only the user increased the recursion limit."] # [doc = ""] # [doc = " We return those obligations here and mention them in the"] # [doc = " error message."] overflowing_predicates : Vec < ty :: Predicate < 'tcx > > , } , }}}

macro_rules! impl_intersection_has_impossible_obligation_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function impl_intersection_has_impossible_obligation in module {}", module_path!());
    };
}

mkfn!{
    impl_intersection_has_impossible_obligation_introspect!();
    # [doc = " Check if both impls can be satisfied by a common type by considering whether"] # [doc = " any of either impl's obligations is not known to hold."] # [doc = ""] # [doc = " For example, given these two impls:"] # [doc = "     `impl From<MyLocalType> for Box<dyn Error>` (in my crate)"] # [doc = "     `impl<E> From<E> for Box<dyn Error> where E: Error` (in libstd)"] # [doc = ""] # [doc = " After replacing both impl headers with inference vars (which happens before"] # [doc = " this function is called), we get:"] # [doc = "     `Box<dyn Error>: From<MyLocalType>`"] # [doc = "     `Box<dyn Error>: From<?E>`"] # [doc = ""] # [doc = " This gives us `?E = MyLocalType`. We then certainly know that `MyLocalType: Error`"] # [doc = " never holds in intercrate mode since a local impl does not exist, and a"] # [doc = " downstream impl cannot be added -- therefore can consider the intersection"] # [doc = " of the two impls above to be empty."] # [doc = ""] # [doc = " Importantly, this works even if there isn't a `impl !Error for MyLocalType`."] # [instrument (level = "debug" , skip (selcx) , ret)] fn impl_intersection_has_impossible_obligation < 'a , 'cx , 'tcx > (selcx : & mut SelectionContext < 'cx , 'tcx > , obligations : & 'a [PredicateObligation < 'tcx >] ,) -> IntersectionHasImpossibleObligations < 'tcx > { let infcx = selcx . infcx ; if infcx . next_trait_solver () { if ! obligations . iter () . all (| o | { < & SolverDelegate < 'tcx > > :: from (infcx) . root_goal_may_hold_with_depth (8 , Goal :: new (infcx . tcx , o . param_env , o . predicate)) }) { return IntersectionHasImpossibleObligations :: Yes ; } let ocx = ObligationCtxt :: new (infcx) ; ocx . register_obligations (obligations . iter () . cloned ()) ; let hard_errors = ocx . select_where_possible () ; if ! hard_errors . is_empty () { assert ! (hard_errors . iter () . all (| e | e . is_true_error ()) , "should not have detected ambiguity during first pass") ; return IntersectionHasImpossibleObligations :: Yes ; } let ambiguities = ocx . into_pending_obligations () ; let ocx = ObligationCtxt :: new_with_diagnostics (infcx) ; ocx . register_obligations (ambiguities) ; let errors_and_ambiguities = ocx . select_all_or_error () ; let (errors , ambiguities) : (Vec < _ > , Vec < _ >) = errors_and_ambiguities . into_iter () . partition (| error | error . is_true_error ()) ; assert ! (errors . is_empty () , "should not have ambiguities during second pass") ; IntersectionHasImpossibleObligations :: No { overflowing_predicates : ambiguities . into_iter () . filter (| error | { matches ! (error . code , FulfillmentErrorCode :: Ambiguity { overflow : Some (true) }) }) . map (| e | infcx . resolve_vars_if_possible (e . obligation . predicate)) . collect () , } } else { for obligation in obligations { let evaluation_result = selcx . evaluate_root_obligation (obligation) ; match evaluation_result { Ok (result) => { if ! result . may_apply () { return IntersectionHasImpossibleObligations :: Yes ; } } Err (_overflow) => { } } } IntersectionHasImpossibleObligations :: No { overflowing_predicates : Vec :: new () } } }
}

macro_rules! impl_intersection_has_negative_obligation_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function impl_intersection_has_negative_obligation in module {}", module_path!());
    };
}

mkfn!{
    impl_intersection_has_negative_obligation_introspect!();
    # [doc = " Check if both impls can be satisfied by a common type by considering whether"] # [doc = " any of first impl's obligations is known not to hold *via a negative predicate*."] # [doc = ""] # [doc = " For example, given these two impls:"] # [doc = "     `struct MyCustomBox<T: ?Sized>(Box<T>);`"] # [doc = "     `impl From<&str> for MyCustomBox<dyn Error>` (in my crate)"] # [doc = "     `impl<E> From<E> for MyCustomBox<dyn Error> where E: Error` (in my crate)"] # [doc = ""] # [doc = " After replacing the second impl's header with inference vars, we get:"] # [doc = "     `MyCustomBox<dyn Error>: From<&str>`"] # [doc = "     `MyCustomBox<dyn Error>: From<?E>`"] # [doc = ""] # [doc = " This gives us `?E = &str`. We then try to prove the first impl's predicates"] # [doc = " after negating, giving us `&str: !Error`. This is a negative impl provided by"] # [doc = " libstd, and therefore we can guarantee for certain that libstd will never add"] # [doc = " a positive impl for `&str: Error` (without it being a breaking change)."] fn impl_intersection_has_negative_obligation (tcx : TyCtxt < '_ > , impl1_def_id : DefId , impl2_def_id : DefId ,) -> bool { debug ! ("negative_impl(impl1_def_id={:?}, impl2_def_id={:?})" , impl1_def_id , impl2_def_id) ; let ref infcx = tcx . infer_ctxt () . with_next_trait_solver (true) . build (TypingMode :: Coherence) ; let root_universe = infcx . universe () ; assert_eq ! (root_universe , ty :: UniverseIndex :: ROOT) ; let impl1_header = fresh_impl_header (infcx , impl1_def_id) ; let param_env = ty :: EarlyBinder :: bind (tcx . param_env (impl1_def_id)) . instantiate (tcx , impl1_header . impl_args) ; let impl2_header = fresh_impl_header (infcx , impl2_def_id) ; let Some (equate_obligations) = equate_impl_headers (infcx , param_env , & impl1_header , & impl2_header) else { return false ; } ; drop (equate_obligations) ; drop (infcx . take_registered_region_obligations ()) ; drop (infcx . take_registered_region_assumptions ()) ; drop (infcx . take_and_reset_region_constraints ()) ; plug_infer_with_placeholders (infcx , root_universe , (impl1_header . impl_args , impl2_header . impl_args) ,) ; let param_env = infcx . resolve_vars_if_possible (param_env) ; util :: elaborate (tcx , tcx . predicates_of (impl2_def_id) . instantiate (tcx , impl2_header . impl_args)) . elaborate_sized () . any (| (clause , _) | try_prove_negated_where_clause (infcx , clause , param_env)) }
}

macro_rules! plug_infer_with_placeholders_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function plug_infer_with_placeholders in module {}", module_path!());
    };
}

mkfn!{
    plug_infer_with_placeholders_introspect!();
    fn plug_infer_with_placeholders < 'tcx > (infcx : & InferCtxt < 'tcx > , universe : ty :: UniverseIndex , value : impl TypeVisitable < TyCtxt < 'tcx > > ,) { struct PlugInferWithPlaceholder < 'a , 'tcx > { infcx : & 'a InferCtxt < 'tcx > , universe : ty :: UniverseIndex , var : ty :: BoundVar , } impl < 'tcx > PlugInferWithPlaceholder < '_ , 'tcx > { fn next_var (& mut self) -> ty :: BoundVar { let var = self . var ; self . var = self . var + 1 ; var } } impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for PlugInferWithPlaceholder < '_ , 'tcx > { fn visit_ty (& mut self , ty : Ty < 'tcx >) { let ty = self . infcx . shallow_resolve (ty) ; if ty . is_ty_var () { let Ok (InferOk { value : () , obligations }) = self . infcx . at (& ObligationCause :: dummy () , ty :: ParamEnv :: empty ()) . eq (DefineOpaqueTypes :: Yes , ty , Ty :: new_placeholder (self . infcx . tcx , ty :: Placeholder { universe : self . universe , bound : ty :: BoundTy { var : self . next_var () , kind : ty :: BoundTyKind :: Anon , } , } ,) ,) else { bug ! ("we always expect to be able to plug an infer var with placeholder") } ; assert_eq ! (obligations . len () , 0) ; } else { ty . super_visit_with (self) ; } } fn visit_const (& mut self , ct : ty :: Const < 'tcx >) { let ct = self . infcx . shallow_resolve_const (ct) ; if ct . is_ct_infer () { let Ok (InferOk { value : () , obligations }) = self . infcx . at (& ObligationCause :: dummy () , ty :: ParamEnv :: empty ()) . eq (DefineOpaqueTypes :: Yes , ct , ty :: Const :: new_placeholder (self . infcx . tcx , ty :: Placeholder { universe : self . universe , bound : ty :: BoundConst { var : self . next_var () } , } ,) ,) else { bug ! ("we always expect to be able to plug an infer var with placeholder") } ; assert_eq ! (obligations . len () , 0) ; } else { ct . super_visit_with (self) ; } } fn visit_region (& mut self , r : ty :: Region < 'tcx >) { if let ty :: ReVar (vid) = r . kind () { let r = self . infcx . inner . borrow_mut () . unwrap_region_constraints () . opportunistic_resolve_var (self . infcx . tcx , vid) ; if r . is_var () { let Ok (InferOk { value : () , obligations }) = self . infcx . at (& ObligationCause :: dummy () , ty :: ParamEnv :: empty ()) . eq (DefineOpaqueTypes :: Yes , r , ty :: Region :: new_placeholder (self . infcx . tcx , ty :: Placeholder { universe : self . universe , bound : ty :: BoundRegion { var : self . next_var () , kind : ty :: BoundRegionKind :: Anon , } , } ,) ,) else { bug ! ("we always expect to be able to plug an infer var with placeholder") } ; assert_eq ! (obligations . len () , 0) ; } } } } value . visit_with (& mut PlugInferWithPlaceholder { infcx , universe , var : ty :: BoundVar :: ZERO }) ; }
}

macro_rules! try_prove_negated_where_clause_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_prove_negated_where_clause in module {}", module_path!());
    };
}

mkfn!{
    try_prove_negated_where_clause_introspect!();
    fn try_prove_negated_where_clause < 'tcx > (root_infcx : & InferCtxt < 'tcx > , clause : ty :: Clause < 'tcx > , param_env : ty :: ParamEnv < 'tcx > ,) -> bool { let Some (negative_predicate) = clause . as_predicate () . flip_polarity (root_infcx . tcx) else { return false ; } ; let ref infcx = root_infcx . fork_with_typing_mode (TypingMode :: non_body_analysis ()) ; let ocx = ObligationCtxt :: new (infcx) ; ocx . register_obligation (Obligation :: new (infcx . tcx , ObligationCause :: dummy () , param_env , negative_predicate ,)) ; if ! ocx . select_all_or_error () . is_empty () { return false ; } let errors = ocx . resolve_regions (CRATE_DEF_ID , param_env , []) ; if ! errors . is_empty () { return false ; } true }
}

macro_rules! compute_intercrate_ambiguity_causes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_intercrate_ambiguity_causes in module {}", module_path!());
    };
}

mkfn!{
    compute_intercrate_ambiguity_causes_introspect!();
    # [doc = " Compute the `intercrate_ambiguity_causes` for the new solver using"] # [doc = " \"proof trees\"."] # [doc = ""] # [doc = " This is a bit scuffed but seems to be good enough, at least"] # [doc = " when looking at UI tests. Given that it is only used to improve"] # [doc = " diagnostics this is good enough. We can always improve it once there"] # [doc = " are test cases where it is currently not enough."] fn compute_intercrate_ambiguity_causes < 'tcx > (infcx : & InferCtxt < 'tcx > , obligations : & [PredicateObligation < 'tcx >] ,) -> FxIndexSet < IntercrateAmbiguityCause < 'tcx > > { let mut causes : FxIndexSet < IntercrateAmbiguityCause < 'tcx > > = Default :: default () ; for obligation in obligations { search_ambiguity_causes (infcx , obligation . as_goal () , & mut causes) ; } causes }
}
mkitem!{mkstruct!{struct AmbiguityCausesVisitor < 'a , 'tcx > { cache : FxHashSet < Goal < 'tcx , ty :: Predicate < 'tcx > > > , causes : & 'a mut FxIndexSet < IntercrateAmbiguityCause < 'tcx > > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > ProofTreeVisitor < 'tcx > for AmbiguityCausesVisitor < 'a , 'tcx > { fn span (& self) -> Span { DUMMY_SP } fn visit_goal (& mut self , goal : & InspectGoal < '_ , 'tcx >) { if ! self . cache . insert (goal . goal ()) { return ; } let infcx = goal . infcx () ; for cand in goal . candidates () { cand . visit_nested_in_probe (self) ; } match goal . result () { Ok (Certainty :: Yes) | Err (NoSolution) => return , Ok (Certainty :: Maybe (_)) => { } } let Goal { param_env , predicate } = goal . goal () ; let trait_ref = match predicate . kind () . no_bound_vars () { Some (ty :: PredicateKind :: Clause (ty :: ClauseKind :: Trait (tr))) => tr . trait_ref , Some (ty :: PredicateKind :: Clause (ty :: ClauseKind :: Projection (proj))) if matches ! (infcx . tcx . def_kind (proj . projection_term . def_id) , DefKind :: AssocTy | DefKind :: AssocConst) => { proj . projection_term . trait_ref (infcx . tcx) } _ => return , } ; if trait_ref . references_error () { return ; } let mut candidates = goal . candidates () ; for cand in goal . candidates () { if let inspect :: ProbeKind :: TraitCandidate { source : CandidateSource :: Impl (def_id) , result : Ok (_) , } = cand . kind () && let ty :: ImplPolarity :: Reservation = infcx . tcx . impl_polarity (def_id) { let message = infcx . tcx . get_attr (def_id , sym :: rustc_reservation_impl) . and_then (| a | a . value_str ()) ; if let Some (message) = message { self . causes . insert (IntercrateAmbiguityCause :: ReservationImpl { message }) ; } } } let Some (cand) = candidates . pop () else { return ; } ; let inspect :: ProbeKind :: TraitCandidate { source : CandidateSource :: CoherenceUnknowable , result : Ok (_) , } = cand . kind () else { return ; } ; let lazily_normalize_ty = | mut ty : Ty < 'tcx > | { if matches ! (ty . kind () , ty :: Alias (..)) { let ocx = ObligationCtxt :: new (infcx) ; ty = ocx . structurally_normalize_ty (& ObligationCause :: dummy () , param_env , ty) . map_err (| _ | ()) ? ; if ! ocx . select_where_possible () . is_empty () { return Err (()) ; } } Ok (ty) } ; infcx . probe (| _ | { let conflict = match trait_ref_is_knowable (infcx , trait_ref , lazily_normalize_ty) { Err (()) => return , Ok (Ok (())) => { warn ! ("expected an unknowable trait ref: {trait_ref:?}") ; return ; } Ok (Err (conflict)) => conflict , } ; let non_intercrate_infcx = infcx . fork_with_typing_mode (TypingMode :: non_body_analysis ()) ; if non_intercrate_infcx . predicate_may_hold (& Obligation :: new (infcx . tcx , ObligationCause :: dummy () , param_env , predicate ,)) { return ; } let trait_ref = deeply_normalize_for_diagnostics (infcx , param_env , trait_ref) ; let self_ty = trait_ref . self_ty () ; let self_ty = self_ty . has_concrete_skeleton () . then (| | self_ty) ; self . causes . insert (match conflict { Conflict :: Upstream => { IntercrateAmbiguityCause :: UpstreamCrateUpdate { trait_ref , self_ty } } Conflict :: Downstream => { IntercrateAmbiguityCause :: DownstreamCrate { trait_ref , self_ty } } }) ; }) ; } }}}

macro_rules! search_ambiguity_causes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function search_ambiguity_causes in module {}", module_path!());
    };
}

mkfn!{
    search_ambiguity_causes_introspect!();
    fn search_ambiguity_causes < 'tcx > (infcx : & InferCtxt < 'tcx > , goal : Goal < 'tcx , ty :: Predicate < 'tcx > > , causes : & mut FxIndexSet < IntercrateAmbiguityCause < 'tcx > > ,) { infcx . probe (| _ | { infcx . visit_proof_tree (goal , & mut AmbiguityCausesVisitor { cache : Default :: default () , causes } ,) }) ; }
}
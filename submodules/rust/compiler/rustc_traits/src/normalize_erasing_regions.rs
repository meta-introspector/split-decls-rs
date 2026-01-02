mkuse!{use rustc_infer :: infer :: TyCtxtInferExt ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: traits :: query :: NoSolution ;}
mkuse!{use rustc_middle :: ty :: { self , PseudoCanonicalInput , TyCtxt , TypeFoldable , TypeVisitableExt } ;}
mkuse!{use rustc_trait_selection :: traits :: query :: normalize :: QueryNormalizeExt ;}
mkuse!{use rustc_trait_selection :: traits :: { Normalized , ObligationCause } ;}
mkuse!{use tracing :: debug ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (p : & mut Providers) { * p = Providers { try_normalize_generic_arg_after_erasing_regions : | tcx , goal | { debug ! ("try_normalize_generic_arg_after_erasing_regions(goal={:#?}" , goal) ; try_normalize_after_erasing_regions (tcx , goal) } , .. * p } ; }
}

macro_rules! try_normalize_after_erasing_regions_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_normalize_after_erasing_regions in module {}", module_path!());
    };
}

mkfn!{
    try_normalize_after_erasing_regions_introspect!();
    fn try_normalize_after_erasing_regions < 'tcx , T : TypeFoldable < TyCtxt < 'tcx > > + PartialEq + Copy > (tcx : TyCtxt < 'tcx > , goal : PseudoCanonicalInput < 'tcx , T > ,) -> Result < T , NoSolution > { let PseudoCanonicalInput { typing_env , value } = goal ; let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (typing_env) ; let cause = ObligationCause :: dummy () ; match infcx . at (& cause , param_env) . query_normalize (value) { Ok (Normalized { value : normalized_value , obligations : normalized_obligations }) => { assert_eq ! (normalized_obligations . iter () . find (| p | not_outlives_predicate (p . predicate)) , None ,) ; let resolved_value = infcx . resolve_vars_if_possible (normalized_value) ; debug_assert_eq ! (normalized_value , resolved_value) ; let erased = infcx . tcx . erase_and_anonymize_regions (resolved_value) ; debug_assert ! (! erased . has_infer () , "{erased:?}") ; Ok (erased) } Err (NoSolution) => Err (NoSolution) , } }
}

macro_rules! not_outlives_predicate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function not_outlives_predicate in module {}", module_path!());
    };
}

mkfn!{
    not_outlives_predicate_introspect!();
    fn not_outlives_predicate (p : ty :: Predicate < '_ >) -> bool { match p . kind () . skip_binder () { ty :: PredicateKind :: Clause (ty :: ClauseKind :: RegionOutlives (..)) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: TypeOutlives (..)) => false , ty :: PredicateKind :: Clause (ty :: ClauseKind :: Trait (..)) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: Projection (..)) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: HostEffect (..)) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: ConstArgHasType (..)) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: UnstableFeature (_)) | ty :: PredicateKind :: NormalizesTo (..) | ty :: PredicateKind :: AliasRelate (..) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: WellFormed (..)) | ty :: PredicateKind :: DynCompatible (..) | ty :: PredicateKind :: Subtype (..) | ty :: PredicateKind :: Coerce (..) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: ConstEvaluatable (..)) | ty :: PredicateKind :: ConstEquate (..) | ty :: PredicateKind :: Ambiguous => true , } }
}
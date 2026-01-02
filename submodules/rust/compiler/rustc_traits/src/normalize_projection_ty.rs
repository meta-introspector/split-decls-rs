mkuse!{use rustc_infer :: infer :: TyCtxtInferExt ;}
mkuse!{use rustc_infer :: infer :: canonical :: { Canonical , QueryResponse } ;}
mkuse!{use rustc_infer :: traits :: PredicateObligations ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: { ParamEnvAnd , TyCtxt } ;}
mkuse!{use rustc_trait_selection :: error_reporting :: InferCtxtErrorExt ;}
mkuse!{use rustc_trait_selection :: infer :: InferCtxtBuilderExt ;}
mkuse!{use rustc_trait_selection :: traits :: query :: normalize :: NormalizationResult ;}
mkuse!{use rustc_trait_selection :: traits :: query :: { CanonicalAliasGoal , NoSolution } ;}
mkuse!{use rustc_trait_selection :: traits :: { self , ObligationCause , ScrubbedTraitError , SelectionContext } ;}
mkuse!{use tracing :: debug ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (p : & mut Providers) { * p = Providers { normalize_canonicalized_projection_ty , normalize_canonicalized_free_alias , normalize_canonicalized_inherent_projection_ty , .. * p } ; }
}

macro_rules! normalize_canonicalized_projection_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function normalize_canonicalized_projection_ty in module {}", module_path!());
    };
}

mkfn!{
    normalize_canonicalized_projection_ty_introspect!();
    fn normalize_canonicalized_projection_ty < 'tcx > (tcx : TyCtxt < 'tcx > , goal : CanonicalAliasGoal < 'tcx > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , NormalizationResult < 'tcx > > > , NoSolution > { debug ! ("normalize_canonicalized_projection_ty(goal={:#?})" , goal) ; tcx . infer_ctxt () . enter_canonical_trait_query (& goal , | ocx , ParamEnvAnd { param_env , value : goal } | { debug_assert ! (! ocx . infcx . next_trait_solver ()) ; let selcx = & mut SelectionContext :: new (ocx . infcx) ; let cause = ObligationCause :: dummy () ; let mut obligations = PredicateObligations :: new () ; let answer = traits :: normalize_projection_term (selcx , param_env , goal . into () , cause , 0 , & mut obligations ,) ; ocx . register_obligations (obligations) ; let errors = ocx . select_where_possible () ; if ! errors . is_empty () { if ! tcx . sess . opts . actually_rustdoc { for error in & errors { if let ScrubbedTraitError :: Cycle (cycle) = & error { ocx . infcx . err_ctxt () . report_overflow_obligation_cycle (cycle) ; } } } return Err (NoSolution) ; } Ok (NormalizationResult { normalized_ty : answer . expect_type () }) } ,) }
}

macro_rules! normalize_canonicalized_free_alias_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function normalize_canonicalized_free_alias in module {}", module_path!());
    };
}

mkfn!{
    normalize_canonicalized_free_alias_introspect!();
    fn normalize_canonicalized_free_alias < 'tcx > (tcx : TyCtxt < 'tcx > , goal : CanonicalAliasGoal < 'tcx > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , NormalizationResult < 'tcx > > > , NoSolution > { debug ! ("normalize_canonicalized_free_alias(goal={:#?})" , goal) ; tcx . infer_ctxt () . enter_canonical_trait_query (& goal , | ocx , ParamEnvAnd { param_env , value : goal } | { let obligations = tcx . predicates_of (goal . def_id) . instantiate_own (tcx , goal . args) . map (| (predicate , span) | { traits :: Obligation :: new (tcx , ObligationCause :: dummy_with_span (span) , param_env , predicate ,) } ,) ; ocx . register_obligations (obligations) ; let normalized_ty = tcx . type_of (goal . def_id) . instantiate (tcx , goal . args) ; Ok (NormalizationResult { normalized_ty }) } ,) }
}

macro_rules! normalize_canonicalized_inherent_projection_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function normalize_canonicalized_inherent_projection_ty in module {}", module_path!());
    };
}

mkfn!{
    normalize_canonicalized_inherent_projection_ty_introspect!();
    fn normalize_canonicalized_inherent_projection_ty < 'tcx > (tcx : TyCtxt < 'tcx > , goal : CanonicalAliasGoal < 'tcx > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , NormalizationResult < 'tcx > > > , NoSolution > { debug ! ("normalize_canonicalized_inherent_projection_ty(goal={:#?})" , goal) ; tcx . infer_ctxt () . enter_canonical_trait_query (& goal , | ocx , ParamEnvAnd { param_env , value : goal } | { let selcx = & mut SelectionContext :: new (ocx . infcx) ; let cause = ObligationCause :: dummy () ; let mut obligations = PredicateObligations :: new () ; let answer = traits :: normalize_inherent_projection (selcx , param_env , goal . into () , cause , 0 , & mut obligations ,) ; ocx . register_obligations (obligations) ; Ok (NormalizationResult { normalized_ty : answer . expect_type () }) } ,) }
}
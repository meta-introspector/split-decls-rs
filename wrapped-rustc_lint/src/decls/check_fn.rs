macro_rules! deps {
    () => {
        VisitOpaqueTypes!();
        FunctionalVariances!();
        ParamKind!();
    };
}

macro_rules! check_fn {
    () => {
        deps!();
        fn check_fn (tcx : TyCtxt < '_ > , parent_def_id : LocalDefId) { let sig = tcx . fn_sig (parent_def_id) . instantiate_identity () ; let mut in_scope_parameters = FxIndexMap :: default () ; let mut current_def_id = Some (parent_def_id . to_def_id ()) ; while let Some (def_id) = current_def_id { let generics = tcx . generics_of (def_id) ; for param in & generics . own_params { in_scope_parameters . insert (param . def_id , ParamKind :: Early (param . name , param . index)) ; } current_def_id = generics . parent ; } for bound_var in sig . bound_vars () { let ty :: BoundVariableKind :: Region (ty :: BoundRegionKind :: Named (def_id)) = bound_var else { span_bug ! (tcx . def_span (parent_def_id) , "unexpected non-lifetime binder on fn sig") ; } ; in_scope_parameters . insert (def_id , ParamKind :: Free (def_id)) ; } let sig = tcx . liberate_late_bound_regions (parent_def_id . to_def_id () , sig) ; sig . visit_with (& mut VisitOpaqueTypes { tcx , parent_def_id , in_scope_parameters , seen : Default :: default () , variances : LazyCell :: new (| | { let mut functional_variances = FunctionalVariances { tcx , variances : FxHashMap :: default () , ambient_variance : ty :: Covariant , generics : tcx . generics_of (parent_def_id) , } ; functional_variances . relate (sig , sig) . unwrap () ; functional_variances . variances }) , outlives_env : LazyCell :: new (| | { let typing_env = ty :: TypingEnv :: non_body_analysis (tcx , parent_def_id) ; let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (typing_env) ; let ocx = ObligationCtxt :: new (& infcx) ; let assumed_wf_tys = ocx . assumed_wf_types (param_env , parent_def_id) . unwrap_or_default () ; OutlivesEnvironment :: new (& infcx , parent_def_id , param_env , assumed_wf_tys) }) , }) ; }
    };
}

check_fn!();
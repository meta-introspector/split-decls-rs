macro_rules! deps {
    () => {
        WfCheckingCtxt!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < 'tcx > WfCheckingCtxt < '_ , 'tcx > { # [doc = " Feature gates RFC 2056 -- trivial bounds, checking for global bounds that"] # [doc = " aren't true."] # [instrument (level = "debug" , skip (self))] fn check_false_global_bounds (& mut self) { let tcx = self . ocx . infcx . tcx ; let mut span = tcx . def_span (self . body_def_id) ; let empty_env = ty :: ParamEnv :: empty () ; let predicates_with_span = tcx . predicates_of (self . body_def_id) . predicates . iter () . copied () ; let implied_obligations = traits :: elaborate (tcx , predicates_with_span) ; for (pred , obligation_span) in implied_obligations { match pred . kind () . skip_binder () { ty :: ClauseKind :: WellFormed (..) | ty :: ClauseKind :: UnstableFeature (..) => continue , _ => { } } if pred . is_global () && ! pred . has_type_flags (TypeFlags :: HAS_BINDER_VARS) { let pred = self . normalize (span , None , pred) ; let hir_node = tcx . hir_node_by_def_id (self . body_def_id) ; if let Some (hir :: Generics { predicates , .. }) = hir_node . generics () { span = predicates . iter () . find (| pred | pred . span . contains (obligation_span)) . map (| pred | pred . span) . unwrap_or (obligation_span) ; } let obligation = Obligation :: new (tcx , traits :: ObligationCause :: new (span , self . body_def_id , ObligationCauseCode :: TrivialBound ,) , empty_env , pred ,) ; self . ocx . register_obligation (obligation) ; } } } }
    };
}

impl_137!()
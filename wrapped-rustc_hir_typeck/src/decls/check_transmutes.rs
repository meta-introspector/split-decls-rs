macro_rules! check_transmutes {
    () => {
        pub (crate) fn check_transmutes (tcx : TyCtxt < '_ > , owner : LocalDefId) { assert ! (! tcx . is_typeck_child (owner . to_def_id ())) ; let typeck_results = tcx . typeck (owner) ; let None = typeck_results . tainted_by_errors else { return } ; let typing_env = ty :: TypingEnv :: post_analysis (tcx , owner) ; for & (from , to , hir_id) in & typeck_results . transmutes_to_check { check_transmute (tcx , typing_env , from , to , hir_id) ; } }
    };
}

check_transmutes!()
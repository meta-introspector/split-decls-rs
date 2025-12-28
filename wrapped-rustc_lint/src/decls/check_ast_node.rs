macro_rules! deps {
    () => {
        EarlyCheckNode!();
        EarlyContext!();
        LintStore!();
        RuntimeCombinedEarlyLintPass!();
    };
}

macro_rules! check_ast_node {
    () => {
        deps!();
        pub fn check_ast_node < 'a > (sess : & Session , tcx : Option < TyCtxt < '_ > > , features : & Features , pre_expansion : bool , lint_store : & LintStore , registered_tools : & RegisteredTools , lint_buffer : Option < LintBuffer > , builtin_lints : impl EarlyLintPass + 'static , check_node : impl EarlyCheckNode < 'a > ,) { let context = EarlyContext :: new (sess , features , ! pre_expansion , lint_store , registered_tools , lint_buffer . unwrap_or_default () ,) ; let passes = if pre_expansion { & lint_store . pre_expansion_passes } else { & lint_store . early_passes } ; if passes . is_empty () { check_ast_node_inner (sess , tcx , check_node , context , builtin_lints) ; } else { let mut passes : Vec < _ > = passes . iter () . map (| mk_pass | (mk_pass) ()) . collect () ; passes . push (Box :: new (builtin_lints)) ; let pass = RuntimeCombinedEarlyLintPass { passes : & mut passes [..] } ; check_ast_node_inner (sess , tcx , check_node , context , pass) ; } }
    };
}

check_ast_node!()
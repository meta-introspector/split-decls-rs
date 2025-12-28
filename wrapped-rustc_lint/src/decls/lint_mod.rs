macro_rules! lint_mod {
    () => {
        fn lint_mod (tcx : TyCtxt < '_ > , module_def_id : LocalModDefId) { late_lint_mod (tcx , module_def_id , BuiltinCombinedModuleLateLintPass :: new ()) ; }
    };
}

lint_mod!()
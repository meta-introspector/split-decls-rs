macro_rules! deps {
    () => {
        Runnable!();
    };
}

macro_rules! as_test_runnable {
    () => {
        deps!();
        fn as_test_runnable (sema : & Semantics < '_ , RootDatabase > , fn_def : & ast :: Fn) -> Option < Runnable > { if test_related_attribute_syn (fn_def) . is_some () { let function = sema . to_def (fn_def) ? ; runnable_fn (sema , function) } else { None } }
    };
}

as_test_runnable!();
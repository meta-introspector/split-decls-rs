macro_rules! deps {
    () => {
        EnvSnapshot!();
        ProcMacroSrv!();
    };
}

macro_rules! list {
    () => {
        deps!();
        pub (crate) fn list () -> Vec < String > { let dylib_path = proc_macro_test_dylib_path () ; let env = EnvSnapshot :: default () ; let srv = ProcMacroSrv :: new (& env) ; let res = srv . list_macros (& dylib_path) . unwrap () ; res . into_iter () . map (| (name , kind) | format ! ("{name} [{kind:?}]")) . collect () }
    };
}

list!()
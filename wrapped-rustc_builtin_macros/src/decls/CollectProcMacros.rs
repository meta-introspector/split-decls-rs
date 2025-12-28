macro_rules! deps {
    () => {
        ProcMacro!();
    };
}

macro_rules! CollectProcMacros {
    () => {
        deps!();
        struct CollectProcMacros < 'a > { macros : Vec < ProcMacro > , in_root : bool , dcx : DiagCtxtHandle < 'a > , session : & 'a Session , source_map : & 'a SourceMap , is_proc_macro_crate : bool , is_test_crate : bool , }
    };
}

CollectProcMacros!()
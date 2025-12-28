macro_rules! deps {
    () => {
        BuildScriptOutput!();
        ProcMacroDylibPath!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl BuildScriptOutput { fn is_empty (& self) -> bool { self . cfgs . is_empty () && self . envs . is_empty () && self . out_dir . is_none () && matches ! (self . proc_macro_dylib_path , ProcMacroDylibPath :: NotBuilt | ProcMacroDylibPath :: NotProcMacro) } }
    };
}

impl_29!()
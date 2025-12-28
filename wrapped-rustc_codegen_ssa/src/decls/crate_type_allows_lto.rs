macro_rules! crate_type_allows_lto {
    () => {
        fn crate_type_allows_lto (crate_type : CrateType) -> bool { match crate_type { CrateType :: Executable | CrateType :: Dylib | CrateType :: Staticlib | CrateType :: Cdylib | CrateType :: ProcMacro | CrateType :: Sdylib => true , CrateType :: Rlib => false , } }
    };
}

crate_type_allows_lto!();
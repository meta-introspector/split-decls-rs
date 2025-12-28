macro_rules! crate_export_threshold {
    () => {
        fn crate_export_threshold (crate_type : CrateType) -> SymbolExportLevel { match crate_type { CrateType :: Executable | CrateType :: Staticlib | CrateType :: ProcMacro | CrateType :: Cdylib => { SymbolExportLevel :: C } CrateType :: Rlib | CrateType :: Dylib | CrateType :: Sdylib => SymbolExportLevel :: Rust , } }
    };
}

crate_export_threshold!();
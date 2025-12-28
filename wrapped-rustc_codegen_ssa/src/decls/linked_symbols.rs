macro_rules! linked_symbols {
    () => {
        pub (crate) fn linked_symbols (tcx : TyCtxt < '_ > , crate_type : CrateType ,) -> Vec < (String , SymbolExportKind) > { match crate_type { CrateType :: Executable | CrateType :: ProcMacro | CrateType :: Cdylib | CrateType :: Dylib | CrateType :: Sdylib => () , CrateType :: Staticlib | CrateType :: Rlib => { return Vec :: new () ; } } match tcx . sess . lto () { Lto :: No | Lto :: ThinLocal => { } Lto :: Thin | Lto :: Fat => { return Vec :: new () ; } } let mut symbols = Vec :: new () ; let export_threshold = symbol_export :: crates_export_threshold (& [crate_type]) ; for_each_exported_symbols_include_dep (tcx , crate_type , | symbol , info , cnum | { if info . level . is_below_threshold (export_threshold) && ! tcx . is_compiler_builtins (cnum) || info . used || info . rustc_std_internal_symbol { symbols . push ((symbol_export :: linking_symbol_name_for_instance_in_crate (tcx , symbol , info . kind , cnum ,) , info . kind ,)) ; } }) ; symbols }
    };
}

linked_symbols!()
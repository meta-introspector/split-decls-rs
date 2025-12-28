macro_rules! threshold {
    () => {
        fn threshold (tcx : TyCtxt < '_ >) -> SymbolExportLevel { crates_export_threshold (tcx . crate_types ()) }
    };
}

threshold!()
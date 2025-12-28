macro_rules! crates_export_threshold {
    () => {
        pub fn crates_export_threshold (crate_types : & [CrateType]) -> SymbolExportLevel { if crate_types . iter () . any (| & crate_type | crate_export_threshold (crate_type) == SymbolExportLevel :: Rust) { SymbolExportLevel :: Rust } else { SymbolExportLevel :: C } }
    };
}

crates_export_threshold!()
macro_rules! deps {
    () => {
        RustAnalyzerPackageMetaData!();
    };
}

macro_rules! PackageMetadata {
    () => {
        deps!();
        # [derive (Deserialize , Default)] struct PackageMetadata { # [serde (rename = "rust-analyzer")] rust_analyzer : Option < RustAnalyzerPackageMetaData > , }
    };
}

PackageMetadata!();
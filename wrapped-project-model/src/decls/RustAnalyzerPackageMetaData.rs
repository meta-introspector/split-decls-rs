macro_rules! RustAnalyzerPackageMetaData {
    () => {
        # [derive (Deserialize , Default , Debug , Clone , Eq , PartialEq)] pub struct RustAnalyzerPackageMetaData { pub rustc_private : bool , }
    };
}

RustAnalyzerPackageMetaData!()
macro_rules! CrateSource {
    () => {
        # [derive (Serialize , Deserialize , Debug , Clone , PartialEq , Eq)] struct CrateSource { include_dirs : Vec < Utf8PathBuf > , exclude_dirs : Vec < Utf8PathBuf > , }
    };
}

CrateSource!()
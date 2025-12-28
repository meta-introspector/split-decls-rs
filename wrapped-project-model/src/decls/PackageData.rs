macro_rules! deps {
    () => {
        PackageDependency!();
        ManifestPath!();
        Package!();
        Target!();
        RustAnalyzerPackageMetaData!();
    };
}

macro_rules! PackageData {
    () => {
        deps!();
        # [doc = " Information associated with a cargo crate"] # [derive (Debug , Clone , Eq , PartialEq)] pub struct PackageData { # [doc = " Version given in the `Cargo.toml`"] pub version : semver :: Version , # [doc = " Name as given in the `Cargo.toml`"] pub name : String , # [doc = " Repository as given in the `Cargo.toml`"] pub repository : Option < String > , # [doc = " Path containing the `Cargo.toml`"] pub manifest : ManifestPath , # [doc = " Targets provided by the crate (lib, bin, example, test, ...)"] pub targets : Vec < Target > , # [doc = " Does this package come from the local filesystem (and is editable)?"] pub is_local : bool , # [doc = " Whether this package is a member of the workspace"] pub is_member : bool , # [doc = " List of packages this package depends on"] pub dependencies : Vec < PackageDependency > , # [doc = " Rust edition for this package"] pub edition : Edition , # [doc = " Features provided by the crate, mapped to the features required by that feature."] pub features : FxHashMap < String , Vec < String > > , # [doc = " List of features enabled on this package"] pub active_features : Vec < String > , # [doc = " Package id"] pub id : Arc < PackageId > , # [doc = " Authors as given in the `Cargo.toml`"] pub authors : Vec < String > , # [doc = " Description as given in the `Cargo.toml`"] pub description : Option < String > , # [doc = " Homepage as given in the `Cargo.toml`"] pub homepage : Option < String > , # [doc = " License as given in the `Cargo.toml`"] pub license : Option < String > , # [doc = " License file as given in the `Cargo.toml`"] pub license_file : Option < Utf8PathBuf > , # [doc = " Readme file as given in the `Cargo.toml`"] pub readme : Option < Utf8PathBuf > , # [doc = " Rust version as given in the `Cargo.toml`"] pub rust_version : Option < semver :: Version > , # [doc = " The contents of [package.metadata.rust-analyzer]"] pub metadata : RustAnalyzerPackageMetaData , # [doc = " If this package is a member of the workspace, store all direct and transitive"] # [doc = " dependencies as long as they are workspace members, to track dependency relationships"] # [doc = " between members."] pub all_member_deps : Option < FxHashSet < Package > > , }
    };
}

PackageData!()
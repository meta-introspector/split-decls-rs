// Generated macro for PackageMetadata (struct)
macro_rules! Depcrate_cargo_metadataPackageMetadata {
() => {
// Module: crate::cargo_metadata
// Provides: {"PackageMetadata"}
// Dependencies: {}
# [doc = " Extra data about a package"] # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord)] pub struct PackageMetadata { # [doc = " The license it is under"] pub license : String , # [doc = " The list of authors from the package metadata"] pub authors : Vec < String > , # [doc = " A list of important files from the package, with their contents."] # [doc = ""] # [doc = " This includes *COPYRIGHT*, *NOTICE*, *AUTHOR*, *LICENSE*, and *LICENCE* files, case-insensitive."] pub notices : BTreeMap < String , String > , # [doc = " If this is true, this dep is in the Rust Standard Library"] pub is_in_libstd : Option < bool > , }
};
}

// Generated macro for Package (struct)
macro_rules! Depcrate_core_metadataPackage {
() => {
// Module: crate::core::metadata
// Provides: {"Package"}
// Dependencies: {}
# [doc = " For more information, see the output of"] # [doc = " <https://doc.rust-lang.org/nightly/cargo/commands/cargo-metadata.html>"] # [derive (Debug , Deserialize)] struct Package { name : String , source : Option < String > , manifest_path : String , dependencies : Vec < Dependency > , features : BTreeMap < String , Vec < String > > , }
};
}

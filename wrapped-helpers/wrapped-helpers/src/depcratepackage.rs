// Generated macro for Package (struct)
macro_rules! DepcratePackage {
() => {
// Module: crate
// Provides: {"Package"}
// Dependencies: {}
# [derive (Deserialize)] pub struct Package { pub name : String , pub version : String , pub edition : String , pub publish : Option < bool > , # [serde (rename = "rust-version")] pub rust_version : Option < String > , pub license : Option < String > , pub description : Option < String > , pub repository : Option < String > , pub readme : Option < String > , pub categories : Option < Vec < String > > , pub authors : Option < Vec < String > > , }
};
}

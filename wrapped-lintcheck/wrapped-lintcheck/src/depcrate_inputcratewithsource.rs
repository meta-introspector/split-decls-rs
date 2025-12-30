// Generated macro for CrateWithSource (struct)
macro_rules! Depcrate_inputCrateWithSource {
() => {
// Module: crate::input
// Provides: {"CrateWithSource"}
// Dependencies: {}
# [doc = " Represents an archive we download from crates.io, or a git repo, or a local repo/folder"] # [doc = " Once processed (downloaded/extracted/cloned/copied...), this will be translated into a `Crate`"] # [derive (Debug , Deserialize , Eq , Hash , PartialEq , Ord , PartialOrd)] pub struct CrateWithSource { pub name : String , pub source : CrateSource , pub file_link : String , pub options : Option < Vec < String > > , }
};
}

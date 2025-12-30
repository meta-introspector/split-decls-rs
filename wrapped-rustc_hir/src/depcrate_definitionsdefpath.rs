// Generated macro for DefPath (struct)
macro_rules! Depcrate_definitionsDefPath {
() => {
// Module: crate::definitions
// Provides: {"DefPath"}
// Dependencies: {}
# [derive (Clone , Debug , Encodable , Decodable)] pub struct DefPath { # [doc = " The path leading from the crate root to the item."] pub data : Vec < DisambiguatedDefPathData > , # [doc = " The crate root this path is relative to."] pub krate : CrateNum , }
};
}

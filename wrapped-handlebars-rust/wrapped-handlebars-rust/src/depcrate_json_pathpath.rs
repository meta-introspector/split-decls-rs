// Generated macro for Path (enum)
macro_rules! Depcrate_json_pathPath {
() => {
// Module: crate::json::path
// Provides: {"Path"}
// Dependencies: {}
# [doc = " Represents the Json path in templates."] # [doc = ""] # [doc = " It can be either a local variable like `@first`, `../@index`,"] # [doc = " or a normal relative path like `a/b/c`."] # [derive (PartialEq , Eq , Clone , Debug)] pub enum Path { Relative ((Vec < PathSeg > , String)) , Local ((usize , String , String)) , }
};
}

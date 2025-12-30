// Generated macro for Type (enum)
macro_rules! Depcrate_typesType {
() => {
// Module: crate::types
// Provides: {"Type"}
// Dependencies: {}
# [doc = " SQLite data types."] # [doc = " See [Fundamental Datatypes](https://sqlite.org/c3ref/c_blob.html)."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum Type { # [doc = " NULL"] Null , # [doc = " 64-bit signed integer"] Integer , # [doc = " 64-bit IEEE floating point number"] Real , # [doc = " String"] Text , # [doc = " BLOB"] Blob , }
};
}

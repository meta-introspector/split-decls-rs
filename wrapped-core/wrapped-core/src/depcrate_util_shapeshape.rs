// Generated macro for Shape (enum)
macro_rules! Depcrate_util_shapeShape {
() => {
// Module: crate::util::shape
// Provides: {"Shape"}
// Dependencies: {}
# [doc = " Description of how fields in a struct or variant are syntactically laid out."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum Shape { # [doc = " A set of named fields, e.g. `{ field: String }`."] Named , # [doc = " A list of unnamed fields, e.g. `(String, u64)`."] Tuple , # [doc = " No fields, e.g. `struct Example;`"] Unit , # [doc = " A special case of [`Tuple`](Shape#variant.Tuple) with exactly one field, e.g. `(String)`."] Newtype , }
};
}

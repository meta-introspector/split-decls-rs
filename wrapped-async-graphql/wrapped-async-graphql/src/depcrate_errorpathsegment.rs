// Generated macro for PathSegment (enum)
macro_rules! Depcrate_errorPathSegment {
() => {
// Module: crate::error
// Provides: {"PathSegment"}
// Dependencies: {}
# [doc = " A segment of path to a resolver."] # [doc = ""] # [doc = " This is like [`QueryPathSegment`](enum.QueryPathSegment.html), but owned and"] # [doc = " used as a part of errors instead of during execution."] # [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] # [serde (untagged)] pub enum PathSegment { # [doc = " A field in an object."] Field (String) , # [doc = " An index in a list."] Index (usize) , }
};
}

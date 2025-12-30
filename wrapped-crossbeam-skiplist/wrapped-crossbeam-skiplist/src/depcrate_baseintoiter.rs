// Generated macro for IntoIter (struct)
macro_rules! Depcrate_baseIntoIter {
() => {
// Module: crate::base
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An owning iterator over the entries of a `SkipList`."] pub struct IntoIter < K , V > { # [doc = " The current node."] # [doc = ""] # [doc = " All preceding nods have already been destroyed."] node : * mut Node < K , V > , }
};
}

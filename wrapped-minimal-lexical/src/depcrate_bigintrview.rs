// Generated macro for rview (function)
macro_rules! Depcrate_bigintrview {
() => {
// Module: crate::bigint
// Provides: {"rview"}
// Dependencies: {}
# [doc = " Create a reverse view of the vector for indexing."] # [inline] pub fn rview (x : & [Limb]) -> ReverseView < Limb > { ReverseView { inner : x , } }
};
}

// Generated macro for and_all (function)
macro_rules! Depcrate_size_hintand_all {
() => {
// Module: crate::size_hint
// Provides: {"and_all"}
// Dependencies: {}
# [doc = " Take the sum of all of the given size hints."] # [doc = ""] # [doc = " If `hints` is empty, returns `(0, Some(0))`, aka the size of consuming"] # [doc = " nothing."] # [inline] pub fn and_all (hints : & [(usize , Option < usize >)]) -> (usize , Option < usize >) { hints . iter () . copied () . fold ((0 , Some (0)) , and) }
};
}

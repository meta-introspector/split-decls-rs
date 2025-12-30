// Generated macro for or_all (function)
macro_rules! Depcrate_size_hintor_all {
() => {
// Module: crate::size_hint
// Provides: {"or_all"}
// Dependencies: {}
# [doc = " Take the maximum of the `lhs` and `rhs` size hints."] # [doc = ""] # [doc = " If `hints` is empty, returns `(0, Some(0))`, aka the size of consuming"] # [doc = " nothing."] # [inline] pub fn or_all (hints : & [(usize , Option < usize >)]) -> (usize , Option < usize >) { if let Some (head) = hints . first () . copied () { hints [1 ..] . iter () . copied () . fold (head , or) } else { (0 , Some (0)) } }
};
}

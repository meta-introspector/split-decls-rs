// Generated macro for eq_simple_pathseg (function)
macro_rules! Depcrate_utileq_simple_pathseg {
() => {
// Module: crate::util
// Provides: {"eq_simple_pathseg"}
// Dependencies: {}
# [doc = " Returns true iff lhs matches the rhs."] fn eq_simple_pathseg (lhs : & str , rhs : & CommaPS) -> bool { lhs . split ("::") . filter (| s | ! s . trim () . is_empty ()) . eq (rhs . iter () . map (| ps | ps . ident . to_string ())) }
};
}

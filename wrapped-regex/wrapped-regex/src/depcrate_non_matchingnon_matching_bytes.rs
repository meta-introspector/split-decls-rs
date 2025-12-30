// Generated macro for non_matching_bytes (function)
macro_rules! Depcrate_non_matchingnon_matching_bytes {
() => {
// Module: crate::non_matching
// Provides: {"non_matching_bytes"}
// Dependencies: {}
# [doc = " Return a confirmed set of non-matching bytes from the given expression."] pub (crate) fn non_matching_bytes (expr : & Hir) -> ByteSet { let mut set = ByteSet :: full () ; remove_matching_bytes (expr , & mut set) ; set }
};
}

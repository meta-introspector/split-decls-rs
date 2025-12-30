// Generated macro for impl_50 (impl)
macro_rules! Depcrate_parserimpl_50 {
() => {
// Module: crate::parser
// Provides: {"impl_50"}
// Dependencies: {}
impl Bumpable for char { fn match_end (self , p : & Parser) -> usize { let mut chars = p . chars () ; if chars . next () . map (| c | c == self) . unwrap_or (false) { chars . cur } else { 0 } } }
};
}

// Generated macro for impl_52 (impl)
macro_rules! Depcrate_parserimpl_52 {
() => {
// Module: crate::parser
// Provides: {"impl_52"}
// Dependencies: {}
impl < F : FnMut (char) -> bool > Bumpable for F { fn match_end (mut self , p : & Parser) -> usize { let mut chars = p . chars () ; let mut count = 0 ; while let Some (c) = chars . next () { if ! self (c) { break } count = chars . cur ; } count } }
};
}

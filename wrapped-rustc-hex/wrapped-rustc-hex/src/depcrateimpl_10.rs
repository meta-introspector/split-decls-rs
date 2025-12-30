// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl < 'a , T : Iterator < Item = & 'a u8 > > Iterator for ToHexIter < T > { type Item = char ; fn next (& mut self) -> Option < char > { if let Some (live) = self . live . take () { return Some (live) ; } self . inner . next () . map (| & byte | { let current = CHARS [(byte >> 4) as usize] as char ; self . live = Some (CHARS [(byte & 0xf) as usize] as char) ; current }) } fn size_hint (& self) -> (usize , Option < usize >) { let (a , b) = self . inner . size_hint () ; (a . saturating_mul (2) , b . map (| b | b . saturating_mul (2))) } }
};
}

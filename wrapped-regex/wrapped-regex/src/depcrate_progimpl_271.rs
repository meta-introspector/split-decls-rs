// Generated macro for impl_271 (impl)
macro_rules! Depcrate_progimpl_271 {
() => {
// Module: crate::prog
// Provides: {"impl_271"}
// Dependencies: {}
impl InstRanges { # [doc = " Tests whether the given input character matches this instruction."] pub fn matches (& self , c : Char) -> bool { for r in self . ranges . iter () . take (4) { if c < r . 0 { return false ; } if c <= r . 1 { return true ; } } self . ranges . binary_search_by (| r | { if r . 1 < c { Ordering :: Less } else if r . 0 > c { Ordering :: Greater } else { Ordering :: Equal } }) . is_ok () } # [doc = " Return the number of distinct characters represented by all of the"] # [doc = " ranges."] pub fn num_chars (& self) -> usize { self . ranges . iter () . map (| & (s , e) | 1 + (e as u32) - (s as u32)) . fold (0 , | acc , len | acc + len) as usize } }
};
}

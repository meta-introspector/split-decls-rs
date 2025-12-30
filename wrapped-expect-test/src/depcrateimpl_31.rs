// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl < 'a > Iterator for LinesWithEnds < 'a > { type Item = & 'a str ; fn next (& mut self) -> Option < & 'a str > { if self . text . is_empty () { return None ; } let idx = self . text . find ('\n') . map_or (self . text . len () , | it | it + 1) ; let (res , next) = self . text . split_at (idx) ; self . text = next ; Some (res) } }
};
}

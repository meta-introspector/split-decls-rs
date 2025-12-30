// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl < 'a > Iterator for PktIterator < 'a > { type Item = & 'a [u8] ; fn next (& mut self) -> Option < Self :: Item > { if self . index < self . data . len () { let start = self . index ; if self . index + 4 <= self . data . len () { for i in self . index .. self . data . len () - 4 { if & self . data [i .. i + 4] == b"fuzz" { self . index = i + 4 ; return Some (& self . data [start .. i]) ; } } } self . index = self . data . len () ; Some (& self . data [start ..]) } else { None } } }
};
}

// Generated macro for impl_3 (impl)
macro_rules! Depcrateimpl_3 {
() => {
// Module: crate
// Provides: {"impl_3"}
// Dependencies: {}
impl Iterator for Counter { type Item = u32 ; fn next (& mut self) -> Option < Self :: Item > { if self . count < 5 { self . count += 1 ; Some (self . count) } else { None } } }
};
}

// Generated macro for impl_272 (impl)
macro_rules! Depcrate_string_arrayimpl_272 {
() => {
// Module: crate::string_array
// Provides: {"impl_272"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for IterBytes < 'a > { fn next_back (& mut self) -> Option < & 'a [u8] > { self . range . next_back () . and_then (| i | self . arr . get_bytes (i)) } }
};
}

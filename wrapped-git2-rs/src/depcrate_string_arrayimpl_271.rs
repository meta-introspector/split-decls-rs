// Generated macro for impl_271 (impl)
macro_rules! Depcrate_string_arrayimpl_271 {
() => {
// Module: crate::string_array
// Provides: {"impl_271"}
// Dependencies: {}
impl < 'a > Iterator for IterBytes < 'a > { type Item = & 'a [u8] ; fn next (& mut self) -> Option < & 'a [u8] > { self . range . next () . and_then (| i | self . arr . get_bytes (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
};
}

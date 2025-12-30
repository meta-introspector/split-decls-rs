// Generated macro for impl_709 (impl)
macro_rules! Depcrate_readimpl_709 {
() => {
// Module: crate::read
// Provides: {"impl_709"}
// Dependencies: {}
impl < 'a > StrRead < 'a > { # [doc = " Create a JSON input source to read from a UTF-8 string."] pub fn new (s : & 'a str) -> Self { StrRead { delegate : SliceRead :: new (s . as_bytes ()) , # [cfg (feature = "raw_value")] data : s , } } }
};
}

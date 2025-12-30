// Generated macro for impl_194 (impl)
macro_rules! Depcrateimpl_194 {
() => {
// Module: crate
// Provides: {"impl_194"}
// Dependencies: {}
# [cfg (feature = "utf16_iter")] impl < 'a > IsNormalizedSinkUtf16 < 'a > { pub fn new (slice : & 'a [u16]) -> Self { IsNormalizedSinkUtf16 { expect : slice } } pub fn remaining_len (& self) -> usize { self . expect . len () } }
};
}

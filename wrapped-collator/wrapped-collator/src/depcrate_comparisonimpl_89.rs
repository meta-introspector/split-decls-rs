// Generated macro for impl_89 (impl)
macro_rules! Depcrate_comparisonimpl_89 {
() => {
// Module: crate::comparison
// Provides: {"impl_89"}
// Dependencies: {}
# [cfg (feature = "latin1")] impl Latin1Chars for [u8] { fn latin1_chars (& self) -> impl DoubleEndedIterator < Item = char > { self . iter () . map (| b | char :: from (* b)) } }
};
}

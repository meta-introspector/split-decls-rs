// Generated macro for Latin1Chars (trait)
macro_rules! Depcrate_comparisonLatin1Chars {
() => {
// Module: crate::comparison
// Provides: {"Latin1Chars"}
// Dependencies: {}
# [doc = " Helper trait for getting a `char` iterator from Latin1 data."] # [doc = ""] # [doc = " ✨ *Enabled with the `latin1` Cargo feature.*"] # [cfg (feature = "latin1")] trait Latin1Chars { fn latin1_chars (& self) -> impl DoubleEndedIterator < Item = char > ; }
};
}

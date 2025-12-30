// Generated macro for impl_126 (impl)
macro_rules! Depcrate_utf8_iteratorsimpl_126 {
() => {
// Module: crate::utf8_iterators
// Provides: {"impl_126"}
// Dependencies: {}
impl < U : Borrow < Utf8Char > , I : Iterator < Item = U > > Utf8CharSplitter < U , I > { # [doc = " Extracts the source iterator."] # [doc = ""] # [doc = " Note that `iter.into_inner().to_bytes()` is not a no-op:  "] # [doc = " If the last returned byte from `next()` was not an ASCII character,"] # [doc = " the remaining bytes of that codepoint is lost."] pub fn into_inner (self) -> I { self . inner } }
};
}

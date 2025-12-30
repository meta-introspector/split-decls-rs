// Generated macro for impl_592 (impl)
macro_rules! Depcrate_byte_strimpl_592 {
() => {
// Module: crate::byte_str
// Provides: {"impl_592"}
// Dependencies: {}
impl ops :: Deref for ByteStr { type Target = str ; # [inline] fn deref (& self) -> & str { let b : & [u8] = self . bytes . as_ref () ; unsafe { str :: from_utf8_unchecked (b) } } }
};
}

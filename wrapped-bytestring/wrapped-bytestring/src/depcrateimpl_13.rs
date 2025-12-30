// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl ops :: Deref for ByteString { type Target = str ; # [inline] fn deref (& self) -> & str { let bytes = self . 0 . as_ref () ; unsafe { str :: from_utf8_unchecked (bytes) } } }
};
}

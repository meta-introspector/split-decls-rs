// Generated macro for impl_159 (impl)
macro_rules! Depcrateimpl_159 {
() => {
// Module: crate
// Provides: {"impl_159"}
// Dependencies: {}
impl < 'a > From < & 'a [u8] > for PrivatePkcs1KeyDer < 'a > { fn from (slice : & 'a [u8]) -> Self { Self (Der (BytesInner :: Borrowed (slice))) } }
};
}

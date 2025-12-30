// Generated macro for impl_173 (impl)
macro_rules! Depcrateimpl_173 {
() => {
// Module: crate
// Provides: {"impl_173"}
// Dependencies: {}
impl < 'a > From < & 'a [u8] > for PrivatePkcs8KeyDer < 'a > { fn from (slice : & 'a [u8]) -> Self { Self (Der (BytesInner :: Borrowed (slice))) } }
};
}

// Generated macro for impl_229 (impl)
macro_rules! Depcrateimpl_229 {
() => {
// Module: crate
// Provides: {"impl_229"}
// Dependencies: {}
impl AsRef < [u8] > for BytesInner < '_ > { fn as_ref (& self) -> & [u8] { match & self { # [cfg (feature = "alloc")] BytesInner :: Owned (vec) => vec . as_ref () , BytesInner :: Borrowed (slice) => slice , } } }
};
}

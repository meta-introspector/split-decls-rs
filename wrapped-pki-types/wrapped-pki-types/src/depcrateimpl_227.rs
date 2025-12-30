// Generated macro for impl_227 (impl)
macro_rules! Depcrateimpl_227 {
() => {
// Module: crate
// Provides: {"impl_227"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl BytesInner < '_ > { fn into_owned (self) -> BytesInner < 'static > { BytesInner :: Owned (match self { Self :: Owned (vec) => vec , Self :: Borrowed (slice) => slice . to_vec () , }) } }
};
}

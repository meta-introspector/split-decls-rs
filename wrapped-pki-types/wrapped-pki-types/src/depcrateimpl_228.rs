// Generated macro for impl_228 (impl)
macro_rules! Depcrateimpl_228 {
() => {
// Module: crate
// Provides: {"impl_228"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl zeroize :: Zeroize for BytesInner < 'static > { fn zeroize (& mut self) { match self { BytesInner :: Owned (vec) => vec . zeroize () , BytesInner :: Borrowed (_) => () , } } }
};
}

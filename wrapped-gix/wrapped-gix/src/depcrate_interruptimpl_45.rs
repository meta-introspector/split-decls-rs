// Generated macro for impl_45 (impl)
macro_rules! Depcrate_interruptimpl_45 {
() => {
// Module: crate::interrupt
// Provides: {"impl_45"}
// Dependencies: {}
impl < R > Read < R > where R : io :: Read , { # [doc = " Create a new interruptible reader from `read`."] pub fn new (read : R) -> Self { Read { inner : gix_features :: interrupt :: Read { inner : read , should_interrupt : & IS_INTERRUPTED , } , } } # [doc = " Return the inner reader"] pub fn into_inner (self) -> R { self . inner . inner } }
};
}

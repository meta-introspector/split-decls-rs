// Generated macro for impl_103 (impl)
macro_rules! Depcrate_spi_atomicimpl_103 {
() => {
// Module: crate::spi::atomic
// Provides: {"impl_103"}
// Dependencies: {}
impl < T : Error > Error for AtomicError < T > { fn kind (& self) -> ErrorKind { match self { AtomicError :: Other (e) => e . kind () , _ => ErrorKind :: Other , } } }
};
}

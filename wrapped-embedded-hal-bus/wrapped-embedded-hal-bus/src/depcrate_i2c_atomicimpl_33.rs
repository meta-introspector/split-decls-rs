// Generated macro for impl_33 (impl)
macro_rules! Depcrate_i2c_atomicimpl_33 {
() => {
// Module: crate::i2c::atomic
// Provides: {"impl_33"}
// Dependencies: {}
impl < T : Error > Error for AtomicError < T > { fn kind (& self) -> ErrorKind { match self { AtomicError :: Other (e) => e . kind () , _ => ErrorKind :: Other , } } }
};
}

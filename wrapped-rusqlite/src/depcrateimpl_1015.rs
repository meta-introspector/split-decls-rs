// Generated macro for impl_1015 (impl)
macro_rules! Depcrateimpl_1015 {
() => {
// Module: crate
// Provides: {"impl_1015"}
// Dependencies: {}
impl < 'conn , 'sql > Batch < 'conn , 'sql > { # [doc = " Constructor"] pub fn new (conn : & 'conn Connection , sql : & 'sql str) -> Self { Batch { conn , sql , tail : 0 } } }
};
}

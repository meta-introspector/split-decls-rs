// Generated macro for impl_164 (impl)
macro_rules! Depcrate_bytesimpl_164 {
() => {
// Module: crate::bytes
// Provides: {"impl_164"}
// Dependencies: {}
impl From < Bytes > for Vec < u8 > { fn from (bytes : Bytes) -> Vec < u8 > { let bytes = ManuallyDrop :: new (bytes) ; unsafe { (bytes . vtable . into_vec) (& bytes . data , bytes . ptr , bytes . len) } } }
};
}

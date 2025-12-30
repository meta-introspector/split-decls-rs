// Generated macro for impl_126 (impl)
macro_rules! Depcrate_bytesimpl_126 {
() => {
// Module: crate::bytes
// Provides: {"impl_126"}
// Dependencies: {}
impl hash :: Hash for Bytes { fn hash < H > (& self , state : & mut H) where H : hash :: Hasher , { self . as_slice () . hash (state) ; } }
};
}

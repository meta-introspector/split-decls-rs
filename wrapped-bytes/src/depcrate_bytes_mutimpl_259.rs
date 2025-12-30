// Generated macro for impl_259 (impl)
macro_rules! Depcrate_bytes_mutimpl_259 {
() => {
// Module: crate::bytes_mut
// Provides: {"impl_259"}
// Dependencies: {}
impl hash :: Hash for BytesMut { fn hash < H > (& self , state : & mut H) where H : hash :: Hasher , { let s : & [u8] = self . as_ref () ; s . hash (state) ; } }
};
}

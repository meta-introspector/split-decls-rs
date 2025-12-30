// Generated macro for impl_295 (impl)
macro_rules! Depcrate_bytes_mutimpl_295 {
() => {
// Module: crate::bytes_mut
// Provides: {"impl_295"}
// Dependencies: {}
impl < 'a , T : ? Sized > PartialEq < & 'a T > for BytesMut where BytesMut : PartialEq < T > , { fn eq (& self , other : & & 'a T) -> bool { * self == * * other } }
};
}

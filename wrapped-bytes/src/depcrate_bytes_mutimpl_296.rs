// Generated macro for impl_296 (impl)
macro_rules! Depcrate_bytes_mutimpl_296 {
() => {
// Module: crate::bytes_mut
// Provides: {"impl_296"}
// Dependencies: {}
impl < 'a , T : ? Sized > PartialOrd < & 'a T > for BytesMut where BytesMut : PartialOrd < T > , { fn partial_cmp (& self , other : & & 'a T) -> Option < cmp :: Ordering > { self . partial_cmp (* other) } }
};
}

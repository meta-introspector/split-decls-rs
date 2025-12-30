// Generated macro for impl_81 (impl)
macro_rules! Depcrate_blockimpl_81 {
() => {
// Module: crate::block
// Provides: {"impl_81"}
// Dependencies: {}
impl Hash for Block { # [inline] fn hash < H : Hasher > (& self , hasher : & mut H) { Hash :: hash_slice (& self . into_usize_array () , hasher) ; } }
};
}

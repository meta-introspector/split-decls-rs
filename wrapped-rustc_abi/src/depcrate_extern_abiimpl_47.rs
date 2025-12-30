// Generated macro for impl_47 (impl)
macro_rules! Depcrate_extern_abiimpl_47 {
() => {
// Module: crate::extern_abi
// Provides: {"impl_47"}
// Dependencies: {}
impl Hash for ExternAbi { fn hash < H : Hasher > (& self , state : & mut H) { self . as_str () . hash (state) ; u32 :: from_be_bytes (* b"ABI\0") . hash (state) ; } }
};
}

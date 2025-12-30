// Generated macro for impl_579 (impl)
macro_rules! Depcrate_uriimpl_579 {
() => {
// Module: crate::uri
// Provides: {"impl_579"}
// Dependencies: {}
impl Hash for Uri { fn hash < H > (& self , state : & mut H) where H : Hasher , { if ! self . scheme . inner . is_none () { self . scheme . hash (state) ; state . write_u8 (0xff) ; } if let Some (auth) = self . authority () { auth . hash (state) ; } Hash :: hash_slice (self . path () . as_bytes () , state) ; if let Some (query) = self . query () { b'?' . hash (state) ; Hash :: hash_slice (query . as_bytes () , state) ; } } }
};
}

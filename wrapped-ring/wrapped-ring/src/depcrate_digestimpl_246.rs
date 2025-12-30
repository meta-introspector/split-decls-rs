// Generated macro for impl_246 (impl)
macro_rules! Depcrate_digestimpl_246 {
() => {
// Module: crate::digest
// Provides: {"impl_246"}
// Dependencies: {}
impl core :: fmt :: Debug for Digest { fn fmt (& self , fmt : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (fmt , "{:?}:" , self . algorithm) ? ; debug :: write_hex_bytes (fmt , self . as_ref ()) } }
};
}

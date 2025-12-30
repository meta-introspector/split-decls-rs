// Generated macro for impl_189 (impl)
macro_rules! Depcrate_digestimpl_189 {
() => {
// Module: crate::digest
// Provides: {"impl_189"}
// Dependencies: {}
impl core :: fmt :: Debug for Digest { fn fmt (& self , fmt : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (fmt , "{:?}:" , self . algorithm) ? ; debug :: write_hex_bytes (fmt , self . as_ref ()) } }
};
}

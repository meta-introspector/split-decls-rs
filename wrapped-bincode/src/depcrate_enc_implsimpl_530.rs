// Generated macro for impl_530 (impl)
macro_rules! Depcrate_enc_implsimpl_530 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_530"}
// Dependencies: {}
impl Encode for f64 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match E :: C :: ENDIAN { Endianness :: Big => encoder . writer () . write (& self . to_be_bytes ()) , Endianness :: Little => encoder . writer () . write (& self . to_le_bytes ()) , } } }
};
}

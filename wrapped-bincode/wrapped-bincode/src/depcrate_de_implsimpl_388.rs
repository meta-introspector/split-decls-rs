// Generated macro for impl_388 (impl)
macro_rules! Depcrate_de_implsimpl_388 {
() => {
// Module: crate::de::impls
// Provides: {"impl_388"}
// Dependencies: {}
impl < Context > Decode < Context > for i8 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { decoder . claim_bytes_read (1) ? ; let mut bytes = [0u8 ; 1] ; decoder . reader () . read (& mut bytes) ? ; Ok (bytes [0] as i8) } }
};
}

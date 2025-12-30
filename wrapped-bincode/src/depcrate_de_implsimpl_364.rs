// Generated macro for impl_364 (impl)
macro_rules! Depcrate_de_implsimpl_364 {
() => {
// Module: crate::de::impls
// Provides: {"impl_364"}
// Dependencies: {}
impl < Context > Decode < Context > for u8 { # [inline] fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { decoder . claim_bytes_read (1) ? ; if let Some (buf) = decoder . reader () . peek_read (1) { let byte = buf [0] ; decoder . reader () . consume (1) ; Ok (byte) } else { let mut bytes = [0u8 ; 1] ; decoder . reader () . read (& mut bytes) ? ; Ok (bytes [0]) } } }
};
}

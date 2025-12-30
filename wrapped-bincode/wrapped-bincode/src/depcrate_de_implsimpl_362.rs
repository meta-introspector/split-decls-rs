// Generated macro for impl_362 (impl)
macro_rules! Depcrate_de_implsimpl_362 {
() => {
// Module: crate::de::impls
// Provides: {"impl_362"}
// Dependencies: {}
impl < Context > Decode < Context > for bool { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { match u8 :: decode (decoder) ? { 0 => Ok (false) , 1 => Ok (true) , x => Err (DecodeError :: InvalidBooleanValue (x)) , } } }
};
}

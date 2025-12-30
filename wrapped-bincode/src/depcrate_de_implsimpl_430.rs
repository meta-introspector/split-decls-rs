// Generated macro for impl_430 (impl)
macro_rules! Depcrate_de_implsimpl_430 {
() => {
// Module: crate::de::impls
// Provides: {"impl_430"}
// Dependencies: {}
impl < Context , T > Decode < Context > for Option < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { match super :: decode_option_variant (decoder , core :: any :: type_name :: < Option < T > > ()) ? { Some (_) => { let val = T :: decode (decoder) ? ; Ok (Some (val)) } None => Ok (None) , } } }
};
}

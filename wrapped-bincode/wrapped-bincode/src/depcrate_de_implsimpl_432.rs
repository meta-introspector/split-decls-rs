// Generated macro for impl_432 (impl)
macro_rules! Depcrate_de_implsimpl_432 {
() => {
// Module: crate::de::impls
// Provides: {"impl_432"}
// Dependencies: {}
impl < Context , T , U > Decode < Context > for Result < T , U > where T : Decode < Context > , U : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let is_ok = u32 :: decode (decoder) ? ; match is_ok { 0 => { let t = T :: decode (decoder) ? ; Ok (Ok (t)) } 1 => { let u = U :: decode (decoder) ? ; Ok (Err (u)) } x => Err (DecodeError :: UnexpectedVariant { found : x , allowed : & crate :: error :: AllowedEnumVariants :: Range { max : 1 , min : 0 } , type_name : core :: any :: type_name :: < Result < T , U > > () , }) , } } }
};
}

// Generated macro for impl_431 (impl)
macro_rules! Depcrate_de_implsimpl_431 {
() => {
// Module: crate::de::impls
// Provides: {"impl_431"}
// Dependencies: {}
impl < 'de , T , Context > BorrowDecode < 'de , Context > for Option < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { match super :: decode_option_variant (decoder , core :: any :: type_name :: < Option < T > > ()) ? { Some (_) => { let val = T :: borrow_decode (decoder) ? ; Ok (Some (val)) } None => Ok (None) , } } }
};
}

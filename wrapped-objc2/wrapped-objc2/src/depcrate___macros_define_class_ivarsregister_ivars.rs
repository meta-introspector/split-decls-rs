// Generated macro for register_ivars (function)
macro_rules! Depcrate___macros_define_class_ivarsregister_ivars {
() => {
// Module: crate::__macros::define_class::ivars
// Provides: {"register_ivars"}
// Dependencies: {}
# [doc = " Register the ivars."] # [inline] pub (crate) fn register_ivars < T : DefinedClass > (builder : & mut ClassBuilder , ivars_name : & CStr) { if T :: HAS_IVARS { let ivar_encoding = Encoding :: Array ((mem :: size_of :: < T :: Ivars > () / mem :: align_of :: < T :: Ivars > ()) as u64 , match mem :: align_of :: < T :: Ivars > () { 1 => & u8 :: ENCODING , 2 => & u16 :: ENCODING , 4 => & u32 :: ENCODING , 8 if mem :: align_of :: < u64 > () == 8 => & u64 :: ENCODING , _ => & Encoding :: None , } ,) ; unsafe { builder . add_ivar_inner :: < T :: Ivars > (ivars_name , & ivar_encoding) } ; } }
};
}

// Generated macro for ivar_drop_flag_names (function)
macro_rules! Depcrate___macros_define_class_ivarsivar_drop_flag_names {
() => {
// Module: crate::__macros::define_class::ivars
// Provides: {"ivar_drop_flag_names"}
// Dependencies: {}
# [inline] pub (crate) fn ivar_drop_flag_names < T : DefinedClass > () -> (Cow < 'static , CStr > , Cow < 'static , CStr >) { if cfg ! (feature = "gnustep-1-7") { (CString :: new (format ! ("{}_ivars" , T :: NAME)) . unwrap () . into () , CString :: new (format ! ("{}_drop_flag" , T :: NAME)) . unwrap () . into () ,) } else { unsafe { (CStr :: from_bytes_with_nul_unchecked (b"ivars\0") . into () , CStr :: from_bytes_with_nul_unchecked (b"drop_flag\0") . into () ,) } } }
};
}

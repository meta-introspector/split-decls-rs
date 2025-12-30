// Generated macro for ivars_offset (function)
macro_rules! Depcrate___macros_define_class_ivarsivars_offset {
() => {
// Module: crate::__macros::define_class::ivars
// Provides: {"ivars_offset"}
// Dependencies: {}
# [inline] pub (crate) fn ivars_offset < T : DefinedClass > (cls : & AnyClass , ivars_name : & CStr) -> isize { if T :: HAS_IVARS { fn get_ivar_failed () -> ! { unreachable ! ("failed retrieving instance variable on newly defined class") } cls . instance_variable (ivars_name) . unwrap_or_else (| | get_ivar_failed ()) . offset () } else { 0 } }
};
}

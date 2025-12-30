// Generated macro for drop_flag_offset (function)
macro_rules! Depcrate___macros_define_class_ivarsdrop_flag_offset {
() => {
// Module: crate::__macros::define_class::ivars
// Provides: {"drop_flag_offset"}
// Dependencies: {}
# [inline] pub (crate) fn drop_flag_offset < T : DefinedClass > (cls : & AnyClass , drop_flag_name : & CStr) -> isize { if T :: HAS_DROP_FLAG { fn get_drop_flag_failed () -> ! { unreachable ! ("failed retrieving drop flag instance variable on newly defined class") } cls . instance_variable (drop_flag_name) . unwrap_or_else (| | get_drop_flag_failed ()) . offset () } else { 0 } }
};
}

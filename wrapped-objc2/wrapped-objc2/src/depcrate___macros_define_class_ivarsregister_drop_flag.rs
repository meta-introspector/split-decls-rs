// Generated macro for register_drop_flag (function)
macro_rules! Depcrate___macros_define_class_ivarsregister_drop_flag {
() => {
// Module: crate::__macros::define_class::ivars
// Provides: {"register_drop_flag"}
// Dependencies: {}
# [doc = " Register the drop flag ivar."] # [inline] pub (crate) fn register_drop_flag < T : DefinedClass > (builder : & mut ClassBuilder , drop_flag_name : & CStr ,) { if T :: HAS_DROP_FLAG { builder . add_ivar :: < DropFlag > (drop_flag_name) ; } }
};
}

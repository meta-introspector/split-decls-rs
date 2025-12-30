// Generated macro for ptr_to_drop_flag (function)
macro_rules! Depcrate___macros_define_class_ivarsptr_to_drop_flag {
() => {
// Module: crate::__macros::define_class::ivars
// Provides: {"ptr_to_drop_flag"}
// Dependencies: {}
# [doc = " Helper function for getting a pointer to the drop flag."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The pointer must be valid and have an initialized drop flag."] # [inline] unsafe fn ptr_to_drop_flag < T : DefinedClass > (ptr : NonNull < T >) -> * mut DropFlag { debug_assert ! (T :: HAS_DROP_FLAG , "type did not have drop flag") ; unsafe { AnyObject :: ivar_at_offset :: < DropFlag > (ptr . cast () , T :: __drop_flag_offset ()) . as_ptr () } }
};
}

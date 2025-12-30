// Generated macro for ptr_to_ivar (function)
macro_rules! Depcrate___macros_define_class_ivarsptr_to_ivar {
() => {
// Module: crate::__macros::define_class::ivars
// Provides: {"ptr_to_ivar"}
// Dependencies: {}
# [doc = " Helper function for getting a pointer to the instance variable."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The pointer must be valid, and the instance variable offset (if it has"] # [doc = " any) must have been initialized."] # [inline] unsafe fn ptr_to_ivar < T : ? Sized + DefinedClass > (ptr : NonNull < T >) -> NonNull < T :: Ivars > { unsafe { AnyObject :: ivar_at_offset :: < T :: Ivars > (ptr . cast () , T :: __ivars_offset ()) } }
};
}

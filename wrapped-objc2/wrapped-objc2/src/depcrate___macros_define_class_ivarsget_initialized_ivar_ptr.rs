// Generated macro for get_initialized_ivar_ptr (function)
macro_rules! Depcrate___macros_define_class_ivarsget_initialized_ivar_ptr {
() => {
// Module: crate::__macros::define_class::ivars
// Provides: {"get_initialized_ivar_ptr"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " The pointer must be valid and the instance variables must be initialized."] # [inline] # [track_caller] pub (crate) unsafe fn get_initialized_ivar_ptr < T : DefinedClass > (ptr : NonNull < T > ,) -> NonNull < T :: Ivars > { if T :: HAS_DROP_FLAG && cfg ! (debug_assertions) { match unsafe { * ptr_to_drop_flag (ptr) } { DropFlag :: Allocated => { panic ! ("tried to access uninitialized instance variable") } DropFlag :: InitializedIvars => { } DropFlag :: Finalized => { } } } unsafe { ptr_to_ivar (ptr) } }
};
}

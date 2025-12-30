// Generated macro for initialize_ivars (function)
macro_rules! Depcrate___macros_define_class_ivarsinitialize_ivars {
() => {
// Module: crate::__macros::define_class::ivars
// Provides: {"initialize_ivars"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " The pointer must be a valid, newly allocated instance."] # [inline] # [track_caller] pub (crate) unsafe fn initialize_ivars < T : DefinedClass > (ptr : NonNull < T > , val : T :: Ivars) { if T :: HAS_DROP_FLAG && cfg ! (debug_assertions) { match unsafe { * ptr_to_drop_flag (ptr) } { DropFlag :: Allocated => { } DropFlag :: InitializedIvars => { panic ! ("tried to initialize ivars after they were already initialized") } DropFlag :: Finalized => { panic ! ("tried to initialize ivars on an already initialized object") } } } unsafe { ptr_to_ivar (ptr) . as_ptr () . write (val) } ; if T :: HAS_DROP_FLAG && (mem :: needs_drop :: < T :: Ivars > () || cfg ! (debug_assertions)) { unsafe { ptr_to_drop_flag (ptr) . write (DropFlag :: InitializedIvars) } } }
};
}

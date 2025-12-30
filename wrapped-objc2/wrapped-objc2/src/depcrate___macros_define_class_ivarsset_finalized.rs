// Generated macro for set_finalized (function)
macro_rules! Depcrate___macros_define_class_ivarsset_finalized {
() => {
// Module: crate::__macros::define_class::ivars
// Provides: {"set_finalized"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " The pointer must be valid and finalized (i.e. all super initializers must"] # [doc = " have been run)."] # [inline] # [track_caller] pub (crate) unsafe fn set_finalized < T : DefinedClass > (ptr : NonNull < T >) { if T :: HAS_DROP_FLAG && cfg ! (debug_assertions) { match unsafe { * ptr_to_drop_flag (ptr) } { DropFlag :: Allocated => { panic ! ("tried to finalize an object that was not yet fully initialized") } DropFlag :: InitializedIvars => { } DropFlag :: Finalized => { panic ! ("tried to finalize an already finalized object") } } } if T :: HAS_DROP_FLAG && (mem :: needs_drop :: < T > () || cfg ! (debug_assertions)) { unsafe { ptr_to_drop_flag (ptr) . write (DropFlag :: Finalized) } } }
};
}

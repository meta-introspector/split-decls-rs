// Generated macro for object_downcast_mut (function)
macro_rules! Depcrate_errorobject_downcast_mut {
() => {
// Module: crate::error
// Provides: {"object_downcast_mut"}
// Dependencies: {}
# [cfg (anyhow_no_ptr_addr_of)] unsafe fn object_downcast_mut < E > (e : Mut < ErrorImpl > , target : TypeId) -> Option < Mut < () > > where E : 'static , { if TypeId :: of :: < E > () == target { let unerased_mut = e . cast :: < ErrorImpl < E > > () ; let unerased = unsafe { unerased_mut . deref_mut () } ; Some (Mut :: new (& mut unerased . _object) . cast :: < () > ()) } else { None } }
};
}

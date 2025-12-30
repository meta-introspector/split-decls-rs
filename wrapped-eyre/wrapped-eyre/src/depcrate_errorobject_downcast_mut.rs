// Generated macro for object_downcast_mut (function)
macro_rules! Depcrate_errorobject_downcast_mut {
() => {
// Module: crate::error
// Provides: {"object_downcast_mut"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Requires layout of *e to match ErrorImpl<E>."] unsafe fn object_downcast_mut < E > (e : MutPtr < '_ , ErrorImpl < () > > , target : TypeId ,) -> Option < NonNull < () > > where E : 'static , { if TypeId :: of :: < E > () == target { let unerased = unsafe { e . cast :: < ErrorImpl < E > > () . into_mut () } ; Some (NonNull :: from (& mut (unerased . _object)) . cast :: < () > ()) } else { None } }
};
}

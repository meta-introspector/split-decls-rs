// Generated macro for object_downcast (function)
macro_rules! Depcrate_errorobject_downcast {
() => {
// Module: crate::error
// Provides: {"object_downcast"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Requires layout of *e to match ErrorImpl<E>."] unsafe fn object_downcast < E > (e : RefPtr < '_ , ErrorImpl < () > > , target : TypeId) -> Option < NonNull < () > > where E : 'static , { if TypeId :: of :: < E > () == target { let unerased = unsafe { e . cast :: < ErrorImpl < E > > () . as_ref () } ; Some (NonNull :: from (& (unerased . _object)) . cast :: < () > ()) } else { None } }
};
}

// Generated macro for object_drop_front (function)
macro_rules! Depcrate_errorobject_drop_front {
() => {
// Module: crate::error
// Provides: {"object_drop_front"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Requires layout of *e to match ErrorImpl<E>."] unsafe fn object_drop_front < E > (e : OwnedPtr < ErrorImpl < () > > , target : TypeId) { let _ = target ; let unerased = unsafe { e . cast :: < ErrorImpl < E > > () . into_box () } ; mem :: forget (unerased . _object) }
};
}

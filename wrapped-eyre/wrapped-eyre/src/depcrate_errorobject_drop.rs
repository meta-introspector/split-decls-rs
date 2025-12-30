// Generated macro for object_drop (function)
macro_rules! Depcrate_errorobject_drop {
() => {
// Module: crate::error
// Provides: {"object_drop"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Requires layout of *e to match ErrorImpl<E>."] unsafe fn object_drop < E > (e : OwnedPtr < ErrorImpl < () > >) { let unerased = unsafe { e . cast :: < ErrorImpl < E > > () . into_box () } ; drop (unerased) ; }
};
}

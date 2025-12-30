// Generated macro for object_ref (function)
macro_rules! Depcrate_errorobject_ref {
() => {
// Module: crate::error
// Provides: {"object_ref"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Requires layout of *e to match ErrorImpl<E>."] unsafe fn object_ref < E > (e : RefPtr < '_ , ErrorImpl < () > >) -> & (dyn StdError + Send + Sync + 'static) where E : StdError + Send + Sync + 'static , { & unsafe { e . cast :: < ErrorImpl < E > > () . as_ref () } . _object }
};
}

// Generated macro for object_mut (function)
macro_rules! Depcrate_errorobject_mut {
() => {
// Module: crate::error
// Provides: {"object_mut"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Requires layout of *e to match ErrorImpl<E>."] unsafe fn object_mut < E > (e : MutPtr < '_ , ErrorImpl < () > >) -> & mut (dyn StdError + Send + Sync + 'static) where E : StdError + Send + Sync + 'static , { & mut unsafe { e . cast :: < ErrorImpl < E > > () . into_mut () } . _object }
};
}

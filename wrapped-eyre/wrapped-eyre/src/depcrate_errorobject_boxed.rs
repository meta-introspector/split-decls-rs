// Generated macro for object_boxed (function)
macro_rules! Depcrate_errorobject_boxed {
() => {
// Module: crate::error
// Provides: {"object_boxed"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Requires layout of *e to match ErrorImpl<E>."] unsafe fn object_boxed < E > (e : OwnedPtr < ErrorImpl < () > >) -> Box < dyn StdError + Send + Sync + 'static > where E : StdError + Send + Sync + 'static , { unsafe { e . cast :: < ErrorImpl < E > > () . into_box () } }
};
}

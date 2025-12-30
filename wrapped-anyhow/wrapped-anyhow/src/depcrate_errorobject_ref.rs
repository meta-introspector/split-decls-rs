// Generated macro for object_ref (function)
macro_rules! Depcrate_errorobject_ref {
() => {
// Module: crate::error
// Provides: {"object_ref"}
// Dependencies: {}
unsafe fn object_ref < E > (e : Ref < ErrorImpl >) -> Ref < dyn StdError + Send + Sync + 'static > where E : StdError + Send + Sync + 'static , { let unerased_ref = e . cast :: < ErrorImpl < E > > () ; # [cfg (not (anyhow_no_ptr_addr_of))] return Ref :: from_raw (unsafe { NonNull :: new_unchecked (ptr :: addr_of ! ((* unerased_ref . as_ptr ()) . _object) as * mut E) }) ; # [cfg (anyhow_no_ptr_addr_of)] return Ref :: new (unsafe { & unerased_ref . deref () . _object }) ; }
};
}

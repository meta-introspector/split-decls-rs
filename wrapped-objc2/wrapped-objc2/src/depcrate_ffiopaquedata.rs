// Generated macro for OpaqueData (type)
macro_rules! Depcrate_ffiOpaqueData {
() => {
// Module: crate::ffi
// Provides: {"OpaqueData"}
// Dependencies: {}
# [doc = " We don't know much about the actual structs, so better mark them `!Send`,"] # [doc = " `!Sync`, `!UnwindSafe`, `!RefUnwindSafe`, `!Unpin` and as mutable behind"] # [doc = " shared references."] # [doc = ""] # [doc = " Downstream libraries can always manually opt in to these types afterwards."] # [doc = " (It's also less of a breaking change on our part if we re-add these)."] # [doc = ""] # [doc = " TODO: Replace this with `extern type` to also mark it as `!Sized`."] pub (crate) type OpaqueData = UnsafeCell < PhantomData < (* const UnsafeCell < () > , PhantomPinned) > > ;
};
}

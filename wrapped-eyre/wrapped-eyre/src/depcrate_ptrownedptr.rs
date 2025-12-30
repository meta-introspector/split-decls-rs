// Generated macro for OwnedPtr (struct)
macro_rules! Depcrate_ptrOwnedPtr {
() => {
// Module: crate::ptr
// Provides: {"OwnedPtr"}
// Dependencies: {}
# [doc = " An owned pointer"] # [doc = ""] # [doc = " **NOTE**: Does not deallocate when dropped"] pub (crate) struct OwnedPtr < T : ? Sized > { ptr : NonNull < T > , }
};
}

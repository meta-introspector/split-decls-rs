// Generated macro for OwnedArg (struct)
macro_rules! Depcrate_shims_native_lib_ffiOwnedArg {
() => {
// Module: crate::shims::native_lib::ffi
// Provides: {"OwnedArg"}
// Dependencies: {}
# [doc = " An argument for an FFI call."] # [derive (Debug , Clone)] pub struct OwnedArg { # [doc = " The type descriptor for this argument."] ty : Option < FfiType > , # [doc = " Corresponding bytes for the value."] bytes : Box < [u8] > , }
};
}

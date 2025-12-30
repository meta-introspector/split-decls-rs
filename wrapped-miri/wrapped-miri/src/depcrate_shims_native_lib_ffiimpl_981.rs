// Generated macro for impl_981 (impl)
macro_rules! Depcrate_shims_native_lib_ffiimpl_981 {
() => {
// Module: crate::shims::native_lib::ffi
// Provides: {"impl_981"}
// Dependencies: {}
impl OwnedArg { # [doc = " Instantiates an argument from a type descriptor and bytes."] pub fn new (ty : FfiType , bytes : Box < [u8] >) -> Self { Self { ty : Some (ty) , bytes } } # [doc = " Creates a libffi argument pointer pointing to this argument's bytes."] # [doc = " NB: Since `libffi::middle::Arg` ignores the lifetime of the reference"] # [doc = " it's derived from, it is up to the caller to ensure the `OwnedArg` is"] # [doc = " not dropped before unsafely calling `libffi::middle::Cif::call()`!"] fn ptr (& self) -> ArgPtr { ArgPtr :: new (& self . bytes [0]) } }
};
}

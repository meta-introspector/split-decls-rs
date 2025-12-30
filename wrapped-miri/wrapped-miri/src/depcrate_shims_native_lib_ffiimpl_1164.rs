// Generated macro for impl_1164 (impl)
macro_rules! Depcrate_shims_native_lib_ffiimpl_1164 {
() => {
// Module: crate::shims::native_lib::ffi
// Provides: {"impl_1164"}
// Dependencies: {}
impl OwnedArg { # [doc = " Instantiates an argument from a type descriptor and bytes."] pub fn new (ty : FfiType , bytes : Box < [u8] >) -> Self { Self { ty : Some (ty) , bytes } } }
};
}

// Generated macro for impl_963 (impl)
macro_rules! Depcrate_llvm_ffiimpl_963 {
() => {
// Module: crate::llvm::ffi
// Provides: {"impl_963"}
// Dependencies: {}
impl Bool { pub (crate) const TRUE : Self = Self { value : 1 } ; pub (crate) const FALSE : Self = Self { value : 0 } ; pub (crate) const fn from_bool (rust_bool : bool) -> Self { if rust_bool { Self :: TRUE } else { Self :: FALSE } } # [doc = " Converts this LLVM-C boolean to a Rust `bool`"] pub (crate) fn is_true (self) -> bool { self . value != Self :: FALSE . value } }
};
}

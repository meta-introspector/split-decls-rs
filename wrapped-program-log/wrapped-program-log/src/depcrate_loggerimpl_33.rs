// Generated macro for impl_33 (impl)
macro_rules! Depcrate_loggerimpl_33 {
() => {
// Module: crate::logger
// Provides: {"impl_33"}
// Dependencies: {}
# [doc = " Implement the log trait for the `bool` type."] unsafe impl Log for bool { # [inline] fn debug_with_args (& self , buffer : & mut [MaybeUninit < u8 >] , args : & [Argument]) -> usize { let value = if * self { "true" } else { "false" } ; value . debug_with_args (buffer , args) } # [inline] fn write_with_args (& self , buffer : & mut [MaybeUninit < u8 >] , args : & [Argument]) -> usize { let value = if * self { "true" } else { "false" } ; value . write_with_args (buffer , args) } }
};
}

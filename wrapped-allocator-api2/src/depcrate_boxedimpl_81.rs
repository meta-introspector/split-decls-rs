// Generated macro for impl_81 (impl)
macro_rules! Depcrate_boxedimpl_81 {
() => {
// Module: crate::boxed
// Provides: {"impl_81"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T , const N : usize > From < [T ; N] > for Box < [T] > { # [doc = " Converts a `[T; N]` into a `Box<[T]>`"] # [doc = ""] # [doc = " This conversion moves the array to newly heap-allocated memory."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::boxed::Box;"] # [doc = ""] # [doc = " let boxed: Box<[u8]> = Box::from([4, 2]);"] # [doc = " println!(\"{boxed:?}\");"] # [doc = " ```"] # [inline (always)] fn from (array : [T ; N]) -> Box < [T] > { Box :: slice (Box :: new (array)) } }
};
}

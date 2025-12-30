// Generated macro for WrappingShl (trait)
macro_rules! Depcrate_ops_wrappingWrappingShl {
() => {
// Module: crate::ops::wrapping
// Provides: {"WrappingShl"}
// Dependencies: {}
# [doc = " Performs a left shift that does not panic."] pub trait WrappingShl : Sized + Shl < usize , Output = Self > { # [doc = " Panic-free bitwise shift-left; yields `self << mask(rhs)`,"] # [doc = " where `mask` removes any high order bits of `rhs` that would"] # [doc = " cause the shift to exceed the bitwidth of the type."] # [doc = ""] # [doc = " ```"] # [doc = " use num_traits::WrappingShl;"] # [doc = ""] # [doc = " let x: u16 = 0x0001;"] # [doc = ""] # [doc = " assert_eq!(WrappingShl::wrapping_shl(&x, 0),  0x0001);"] # [doc = " assert_eq!(WrappingShl::wrapping_shl(&x, 1),  0x0002);"] # [doc = " assert_eq!(WrappingShl::wrapping_shl(&x, 15), 0x8000);"] # [doc = " assert_eq!(WrappingShl::wrapping_shl(&x, 16), 0x0001);"] # [doc = " ```"] fn wrapping_shl (& self , rhs : u32) -> Self ; }
};
}

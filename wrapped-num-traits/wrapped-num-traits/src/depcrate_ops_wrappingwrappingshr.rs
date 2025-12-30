// Generated macro for WrappingShr (trait)
macro_rules! Depcrate_ops_wrappingWrappingShr {
() => {
// Module: crate::ops::wrapping
// Provides: {"WrappingShr"}
// Dependencies: {}
# [doc = " Performs a right shift that does not panic."] pub trait WrappingShr : Sized + Shr < usize , Output = Self > { # [doc = " Panic-free bitwise shift-right; yields `self >> mask(rhs)`,"] # [doc = " where `mask` removes any high order bits of `rhs` that would"] # [doc = " cause the shift to exceed the bitwidth of the type."] # [doc = ""] # [doc = " ```"] # [doc = " use num_traits::WrappingShr;"] # [doc = ""] # [doc = " let x: u16 = 0x8000;"] # [doc = ""] # [doc = " assert_eq!(WrappingShr::wrapping_shr(&x, 0),  0x8000);"] # [doc = " assert_eq!(WrappingShr::wrapping_shr(&x, 1),  0x4000);"] # [doc = " assert_eq!(WrappingShr::wrapping_shr(&x, 15), 0x0001);"] # [doc = " assert_eq!(WrappingShr::wrapping_shr(&x, 16), 0x8000);"] # [doc = " ```"] fn wrapping_shr (& self , rhs : u32) -> Self ; }
};
}

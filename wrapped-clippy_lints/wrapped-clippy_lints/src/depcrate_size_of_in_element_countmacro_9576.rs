// Generated macro for macro_9576 (macro)
macro_rules! Depcrate_size_of_in_element_countmacro_9576 {
() => {
// Module: crate::size_of_in_element_count
// Provides: {"macro_9576"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects expressions where"] # [doc = " `size_of::<T>` or `size_of_val::<T>` is used as a"] # [doc = " count of elements of type `T`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " These functions expect a count"] # [doc = " of `T` and not a number of bytes"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,no_run"] # [doc = " # use std::ptr::copy_nonoverlapping;"] # [doc = " const SIZE: usize = 128;"] # [doc = " let x = [2u8; SIZE];"] # [doc = " let mut y = [2u8; SIZE];"] # [doc = " unsafe { copy_nonoverlapping(x.as_ptr(), y.as_mut_ptr(), size_of::<u8>() * SIZE) };"] # [doc = " ```"] # [clippy :: version = "1.50.0"] pub SIZE_OF_IN_ELEMENT_COUNT , correctness , "using `size_of::<T>` or `size_of_val::<T>` where a count of elements of `T` is expected" }
};
}

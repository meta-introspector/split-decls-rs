// Generated macro for Complex (struct)
macro_rules! DepcrateComplex {
() => {
// Module: crate
// Provides: {"Complex"}
// Dependencies: {}
# [doc = " A complex number in Cartesian form."] # [doc = ""] # [doc = " ## Representation and Foreign Function Interface Compatibility"] # [doc = ""] # [doc = " `Complex<T>` is memory layout compatible with an array `[T; 2]`."] # [doc = ""] # [doc = " Note that `Complex<F>` where F is a floating point type is **only** memory"] # [doc = " layout compatible with C's complex types, **not** necessarily calling"] # [doc = " convention compatible.  This means that for FFI you can only pass"] # [doc = " `Complex<F>` behind a pointer, not as a value."] # [doc = ""] # [doc = " ## Examples"] # [doc = ""] # [doc = " Example of extern function declaration."] # [doc = ""] # [doc = " ```"] # [doc = " use num_complex::Complex;"] # [doc = " use std::os::raw::c_int;"] # [doc = ""] # [doc = " extern \"C\" {"] # [doc = "     fn zaxpy_(n: *const c_int, alpha: *const Complex<f64>,"] # [doc = "               x: *const Complex<f64>, incx: *const c_int,"] # [doc = "               y: *mut Complex<f64>, incy: *const c_int);"] # [doc = " }"] # [doc = " ```"] # [derive (PartialEq , Eq , Copy , Clone , Hash , Debug , Default)] # [repr (C)] # [cfg_attr (feature = "rkyv" , derive (rkyv :: Archive , rkyv :: Serialize , rkyv :: Deserialize))] # [cfg_attr (feature = "rkyv" , archive (as = "Complex<T::Archived>"))] # [cfg_attr (feature = "bytecheck" , derive (bytecheck :: CheckBytes))] pub struct Complex < T > { # [doc = " Real portion of the complex number"] pub re : T , # [doc = " Imaginary portion of the complex number"] pub im : T , }
};
}

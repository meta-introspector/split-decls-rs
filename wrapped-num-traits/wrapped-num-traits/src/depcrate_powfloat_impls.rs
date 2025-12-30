// Generated macro for float_impls (module)
macro_rules! Depcrate_powfloat_impls {
() => {
// Module: crate::pow
// Provides: {"float_impls"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "libm"))] mod float_impls { use super :: Pow ; use crate :: Float ; pow_impl ! (f32 , i8 , i32 , < f32 as Float >:: powi) ; pow_impl ! (f32 , u8 , i32 , < f32 as Float >:: powi) ; pow_impl ! (f32 , i16 , i32 , < f32 as Float >:: powi) ; pow_impl ! (f32 , u16 , i32 , < f32 as Float >:: powi) ; pow_impl ! (f32 , i32 , i32 , < f32 as Float >:: powi) ; pow_impl ! (f64 , i8 , i32 , < f64 as Float >:: powi) ; pow_impl ! (f64 , u8 , i32 , < f64 as Float >:: powi) ; pow_impl ! (f64 , i16 , i32 , < f64 as Float >:: powi) ; pow_impl ! (f64 , u16 , i32 , < f64 as Float >:: powi) ; pow_impl ! (f64 , i32 , i32 , < f64 as Float >:: powi) ; pow_impl ! (f32 , f32 , f32 , < f32 as Float >:: powf) ; pow_impl ! (f64 , f32 , f64 , < f64 as Float >:: powf) ; pow_impl ! (f64 , f64 , f64 , < f64 as Float >:: powf) ; }
};
}

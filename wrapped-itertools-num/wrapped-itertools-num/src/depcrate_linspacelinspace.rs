// Generated macro for linspace (function)
macro_rules! Depcrate_linspacelinspace {
() => {
// Module: crate::linspace
// Provides: {"linspace"}
// Dependencies: {}
# [doc = " Return an iterator of evenly spaced floats."] # [doc = ""] # [doc = " The `Linspace` has `n` elements, where the first"] # [doc = " element is `a` and the last element is `b`."] # [doc = ""] # [doc = " Iterator element type is `F`, where `F` must be"] # [doc = " either `f32` or `f64`."] # [doc = ""] # [doc = " ```"] # [doc = " extern crate itertools;"] # [doc = " extern crate itertools_num;"] # [doc = ""] # [doc = " use itertools_num::linspace;"] # [doc = ""] # [doc = " # fn main() {"] # [doc = " itertools::assert_equal(linspace::<f32>(0., 1., 5),"] # [doc = "                         vec![0., 0.25, 0.5, 0.75, 1.0]);"] # [doc = " # }"] # [doc = " ```"] # [inline] pub fn linspace < F > (a : F , b : F , n : usize) -> Linspace < F > where F : Float { let step = if n > 1 { let nf : F = F :: from (n) . unwrap () ; (b - a) / (nf - F :: one ()) } else { F :: zero () } ; Linspace { start : a , step : step , index : 0 , len : n , } }
};
}

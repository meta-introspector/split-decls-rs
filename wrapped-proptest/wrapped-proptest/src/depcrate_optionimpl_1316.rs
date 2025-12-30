// Generated macro for impl_1316 (impl)
macro_rules! Depcrate_optionimpl_1316 {
() => {
// Module: crate::option
// Provides: {"impl_1316"}
// Dependencies: {}
impl Probability { # [doc = " Creates a `Probability` from a `f64`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the probability is outside interval `[0.0, 1.0]`."] pub fn new (prob : f64) -> Self { assert ! (prob >= 0.0 && prob <= 1.0) ; Probability (prob) } # [doc = " Merges self together with some other argument producing a product"] # [doc = " type expected by some implementations of `A: Arbitrary` in"] # [doc = " `A::Parameters`. This can be more ergonomic to work with and may"] # [doc = " help type inference."] pub fn with < X > (self , and : X) -> product_type ! [Self , X] { product_pack ! [self , and] } # [doc = " Merges self together with some other argument generated with a"] # [doc = " default value producing a product type expected by some"] # [doc = " implementations of `A: Arbitrary` in `A::Parameters`."] # [doc = " This can be more ergonomic to work with and may help type inference."] pub fn lift < X : Default > (self) -> product_type ! [Self , X] { self . with (Default :: default ()) } }
};
}

// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl Inflation { pub fn new_disabled () -> Self { Self { initial : 0.0 , terminal : 0.0 , taper : 0.0 , foundation : 0.0 , foundation_term : 0.0 , __unused : 0.0 , } } pub fn new_fixed (validator : f64) -> Self { Self { initial : validator , terminal : validator , taper : 1.0 , foundation : 0.0 , foundation_term : 0.0 , __unused : 0.0 , } } pub fn pico () -> Self { Self :: new_fixed (0.0001) } pub fn full () -> Self { Self { initial : DEFAULT_INITIAL , terminal : DEFAULT_TERMINAL , taper : DEFAULT_TAPER , foundation : 0.0 , foundation_term : 0.0 , __unused : 0.0 , } } # [doc = " inflation rate at year"] pub fn total (& self , year : f64) -> f64 { assert ! (year >= 0.0) ; let tapered = self . initial * ((1.0 - self . taper) . powf (year)) ; if tapered > self . terminal { tapered } else { self . terminal } } # [doc = " portion of total that goes to validators"] pub fn validator (& self , year : f64) -> f64 { self . total (year) - self . foundation (year) } # [doc = " portion of total that goes to foundation"] pub fn foundation (& self , year : f64) -> f64 { if year < self . foundation_term { self . total (year) * self . foundation } else { 0.0 } } }
};
}

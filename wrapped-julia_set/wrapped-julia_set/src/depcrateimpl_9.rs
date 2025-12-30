// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl Complex { fn square (self) -> Complex { let real = (self . real * self . real) - (self . imaginary * self . imaginary) ; let imaginary = 2.0 * self . real * self . imaginary ; Complex { real , imaginary } } fn norm (& self) -> f64 { (self . real * self . real) + (self . imaginary * self . imaginary) } }
};
}

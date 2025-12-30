// Generated macro for impl_609 (impl)
macro_rules! Depcrate_measure_provider_si_prefiximpl_609 {
() => {
// Module: crate::measure::provider::si_prefix
// Provides: {"impl_609"}
// Dependencies: {}
impl SiPrefix { # [doc = " Appends the short representation of the si prefix to the given string."] pub (crate) fn append_short_representation (& self , buff : & mut String) { buff . push (match self . base { Base :: Decimal => 'D' , Base :: Binary => 'B' , }) ; let _infallible = write ! (buff , "{}" , self . power) ; } }
};
}

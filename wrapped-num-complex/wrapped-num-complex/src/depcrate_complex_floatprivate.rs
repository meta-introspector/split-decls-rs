// Generated macro for private (module)
macro_rules! Depcrate_complex_floatprivate {
() => {
// Module: crate::complex_float
// Provides: {"private"}
// Dependencies: {}
mod private { use num_traits :: { Float , FloatConst } ; use crate :: Complex ; pub trait Seal { } impl < T > Seal for T where T : Float + FloatConst { } impl < T : Float + FloatConst > Seal for Complex < T > { } }
};
}

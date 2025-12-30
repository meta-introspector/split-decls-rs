// Generated macro for macro_62 (macro)
macro_rules! Depcrate_float_powmacro_62 {
() => {
// Module: crate::float::pow
// Provides: {"macro_62"}
// Dependencies: {}
intrinsics ! { pub extern "C" fn __powisf2 (a : f32 , b : i32) -> f32 { pow (a , b) } pub extern "C" fn __powidf2 (a : f64 , b : i32) -> f64 { pow (a , b) } # [ppc_alias = __powikf2] # [cfg (f128_enabled)] pub extern "C" fn __powitf2 (a : f128 , b : i32) -> f128 { pow (a , b) } }
};
}

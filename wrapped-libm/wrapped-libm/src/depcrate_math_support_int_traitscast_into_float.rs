// Generated macro for cast_into_float (macro)
macro_rules! Depcrate_math_support_int_traitscast_into_float {
() => {
// Module: crate::math::support::int_traits
// Provides: {"cast_into_float"}
// Dependencies: {}
macro_rules ! cast_into_float { ($ ty : ty) => { # [cfg (f16_enabled)] cast_into_float ! ($ ty ; f16) ; cast_into_float ! ($ ty ; f32 , f64) ; # [cfg (f128_enabled)] cast_into_float ! ($ ty ; f128) ; } ; ($ ty : ty ; $ ($ into : ty) ,*) => { $ (impl CastInto <$ into > for $ ty { fn cast (self) -> $ into { # [cfg (not (feature = "compiler-builtins"))] debug_assert_eq ! (self as $ into as $ ty , self , "inexact float cast") ; self as $ into } fn cast_lossy (self) -> $ into { self as $ into } }) * } ; }
};
}

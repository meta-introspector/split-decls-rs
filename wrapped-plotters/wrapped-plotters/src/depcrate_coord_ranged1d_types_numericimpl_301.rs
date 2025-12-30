// Generated macro for impl_301 (impl)
macro_rules! Depcrate_coord_ranged1d_types_numericimpl_301 {
() => {
// Module: crate::coord::ranged1d::types::numeric
// Provides: {"impl_301"}
// Dependencies: {}
impl ValueFormatter < f32 > for WithKeyPoints < RangedCoordf32 > { fn format (value : & f32) -> String { crate :: data :: float :: FloatPrettyPrinter { allow_scientific : false , min_decimal : 1 , max_decimal : 5 , } . print (* value as f64) } }
};
}

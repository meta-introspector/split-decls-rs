// Generated macro for impl_305 (impl)
macro_rules! Depcrate_coord_ranged1d_types_numericimpl_305 {
() => {
// Module: crate::coord::ranged1d::types::numeric
// Provides: {"impl_305"}
// Dependencies: {}
impl ValueFormatter < f64 > for WithKeyPoints < RangedCoordf64 > { fn format (value : & f64) -> String { crate :: data :: float :: FloatPrettyPrinter { allow_scientific : false , min_decimal : 1 , max_decimal : 5 , } . print (* value) } }
};
}

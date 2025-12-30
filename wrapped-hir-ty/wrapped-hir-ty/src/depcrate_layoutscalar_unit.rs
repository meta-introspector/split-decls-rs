// Generated macro for scalar_unit (function)
macro_rules! Depcrate_layoutscalar_unit {
() => {
// Module: crate::layout
// Provides: {"scalar_unit"}
// Dependencies: {}
fn scalar_unit (dl : & TargetDataLayout , value : Primitive) -> Scalar { Scalar :: Initialized { value , valid_range : WrappingRange :: full (value . size (dl)) } }
};
}

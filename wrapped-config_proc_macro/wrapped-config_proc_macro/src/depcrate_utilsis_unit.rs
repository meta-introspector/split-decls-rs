// Generated macro for is_unit (function)
macro_rules! Depcrate_utilsis_unit {
() => {
// Module: crate::utils
// Provides: {"is_unit"}
// Dependencies: {}
pub fn is_unit (v : & syn :: Variant) -> bool { match v . fields { syn :: Fields :: Unit => true , _ => false , } }
};
}

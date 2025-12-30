// Generated macro for is_time_type (function)
macro_rules! Depcrate_time_subtractionis_time_type {
() => {
// Module: crate::time_subtraction
// Provides: {"is_time_type"}
// Dependencies: {}
# [doc = " Returns true if the type is Duration or Instant"] fn is_time_type (cx : & LateContext < '_ > , ty : Ty < '_ >) -> bool { ty . is_diag_item (cx , sym :: Duration) || ty . is_diag_item (cx , sym :: Instant) }
};
}

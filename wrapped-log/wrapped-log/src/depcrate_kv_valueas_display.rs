// Generated macro for as_display (macro)
macro_rules! Depcrate_kv_valueas_display {
() => {
// Module: crate::kv::value
// Provides: {"as_display"}
// Dependencies: {}
# [doc = " Get a value from a type implementing `std::fmt::Display`."] # [cfg (feature = "kv_unstable")] # [deprecated (note = "use the `key:% = value` macro syntax instead")] # [macro_export] macro_rules ! as_display { ($ capture : expr) => { $ crate :: kv :: Value :: from_display (&$ capture) } ; }
};
}

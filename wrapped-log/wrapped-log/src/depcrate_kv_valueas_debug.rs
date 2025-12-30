// Generated macro for as_debug (macro)
macro_rules! Depcrate_kv_valueas_debug {
() => {
// Module: crate::kv::value
// Provides: {"as_debug"}
// Dependencies: {}
# [doc = " Get a value from a type implementing `std::fmt::Debug`."] # [cfg (feature = "kv_unstable")] # [deprecated (note = "use the `key:? = value` macro syntax instead")] # [macro_export] macro_rules ! as_debug { ($ capture : expr) => { $ crate :: kv :: Value :: from_debug (&$ capture) } ; }
};
}

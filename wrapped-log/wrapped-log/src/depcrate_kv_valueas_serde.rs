// Generated macro for as_serde (macro)
macro_rules! Depcrate_kv_valueas_serde {
() => {
// Module: crate::kv::value
// Provides: {"as_serde"}
// Dependencies: {}
# [cfg (feature = "kv_unstable_serde")] # [deprecated (note = "use the `key:serde = value` macro syntax instead")] # [doc = " Get a value from a type implementing `serde::Serialize`."] # [macro_export] macro_rules ! as_serde { ($ capture : expr) => { $ crate :: kv :: Value :: from_serde (&$ capture) } ; }
};
}

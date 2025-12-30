// Generated macro for as_error (macro)
macro_rules! Depcrate_kv_valueas_error {
() => {
// Module: crate::kv::value
// Provides: {"as_error"}
// Dependencies: {}
# [doc = " Get a value from an error."] # [cfg (feature = "kv_unstable_std")] # [deprecated (note = "use the `key:err = value` macro syntax instead")] # [macro_export] macro_rules ! as_error { ($ capture : expr) => { $ crate :: kv :: Value :: from_dyn_error (&$ capture) } ; }
};
}

// Generated macro for as_sval (macro)
macro_rules! Depcrate_kv_valueas_sval {
() => {
// Module: crate::kv::value
// Provides: {"as_sval"}
// Dependencies: {}
# [doc = " Get a value from a type implementing `sval::Value`."] # [cfg (feature = "kv_unstable_sval")] # [deprecated (note = "use the `key:sval = value` macro syntax instead")] # [macro_export] macro_rules ! as_sval { ($ capture : expr) => { $ crate :: kv :: Value :: from_sval (&$ capture) } ; }
};
}

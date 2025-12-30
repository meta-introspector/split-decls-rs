// Generated macro for impl_810 (impl)
macro_rules! Depcrate_config_snapshot__implsimpl_810 {
() => {
// Module: crate::config::snapshot::_impls
// Provides: {"impl_810"}
// Dependencies: {}
impl Debug for CommitAutoRollback < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . write_str (& self . repo . as_ref () . expect ("still present") . config . resolved . to_string ()) } }
};
}

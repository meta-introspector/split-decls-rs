// Generated macro for impl_37 (impl)
macro_rules! Depcrate_impls_builtins_allocimpl_37 {
() => {
// Module: crate::impls::builtins_alloc
// Provides: {"impl_37"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "alloc" , feature = "use-std"))))] impl < T : ? Sized + Schema + alloc :: borrow :: ToOwned > Schema for alloc :: borrow :: Cow < '_ , T > { const SCHEMA : & 'static DataModelType = T :: SCHEMA ; }
};
}

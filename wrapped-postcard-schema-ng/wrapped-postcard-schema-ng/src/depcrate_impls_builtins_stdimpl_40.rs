// Generated macro for impl_40 (impl)
macro_rules! Depcrate_impls_builtins_stdimpl_40 {
() => {
// Module: crate::impls::builtins_std
// Provides: {"impl_40"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "alloc" , feature = "use-std"))))] impl < T : Schema > Schema for std :: vec :: Vec < T > { const SCHEMA : & 'static DataModelType = & DataModelType :: Seq (T :: SCHEMA) ; }
};
}

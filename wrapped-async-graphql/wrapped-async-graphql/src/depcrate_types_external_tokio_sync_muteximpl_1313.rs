// Generated macro for impl_1313 (impl)
macro_rules! Depcrate_types_external_tokio_sync_muteximpl_1313 {
() => {
// Module: crate::types::external::tokio::sync::mutex
// Provides: {"impl_1313"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : OutputType > OutputType for Mutex < T > { fn type_name () -> Cow < 'static , str > { T :: type_name () } fn create_type_info (registry : & mut registry :: Registry) -> String { < T as OutputType > :: create_type_info (registry) } async fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> ServerResult < Value > { self . lock () . await . resolve (ctx , field) . await } }
};
}

// Generated macro for impl_1185 (impl)
macro_rules! Depcrate_types_external_cowimpl_1185 {
() => {
// Module: crate::types::external::cow
// Provides: {"impl_1185"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T > OutputType for Cow < '_ , T > where T : OutputType + ToOwned + ? Sized , < T as ToOwned > :: Owned : Send + Sync , { fn type_name () -> Cow < 'static , str > { T :: type_name () } fn create_type_info (registry : & mut registry :: Registry) -> String { < T as OutputType > :: create_type_info (registry) } async fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> ServerResult < Value > { self . as_ref () . resolve (ctx , field) . await } }
};
}

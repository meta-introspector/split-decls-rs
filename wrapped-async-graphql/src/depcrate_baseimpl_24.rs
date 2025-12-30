// Generated macro for impl_24 (impl)
macro_rules! Depcrate_baseimpl_24 {
() => {
// Module: crate::base
// Provides: {"impl_24"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : OutputType + ? Sized > OutputType for Weak < T > { fn type_name () -> Cow < 'static , str > { < Option < Arc < T > > as OutputType > :: type_name () } fn create_type_info (registry : & mut Registry) -> String { < Option < Arc < T > > as OutputType > :: create_type_info (registry) } async fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> ServerResult < Value > { self . upgrade () . resolve (ctx , field) . await } }
};
}

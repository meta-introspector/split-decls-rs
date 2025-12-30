// Generated macro for impl_1253 (impl)
macro_rules! Depcrate_types_external_list_hashbrown_hash_setimpl_1253 {
() => {
// Module: crate::types::external::list::hashbrown_hash_set
// Provides: {"impl_1253"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : OutputType + Hash + Eq > OutputType for HashSet < T > { fn type_name () -> Cow < 'static , str > { < StdHashSet < T > as OutputType > :: type_name () } fn qualified_type_name () -> String { < StdHashSet < T > as OutputType > :: qualified_type_name () } fn create_type_info (registry : & mut registry :: Registry) -> String { < StdHashSet < T > as OutputType > :: create_type_info (registry) } async fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> ServerResult < Value > { resolve_list (ctx , field , self , Some (self . len ())) . await } }
};
}

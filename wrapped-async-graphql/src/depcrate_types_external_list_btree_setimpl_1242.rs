// Generated macro for impl_1242 (impl)
macro_rules! Depcrate_types_external_list_btree_setimpl_1242 {
() => {
// Module: crate::types::external::list::btree_set
// Provides: {"impl_1242"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : OutputType + Ord > OutputType for BTreeSet < T > { fn type_name () -> Cow < 'static , str > { Cow :: Owned (format ! ("[{}]" , T :: qualified_type_name ())) } fn qualified_type_name () -> String { format ! ("[{}]!" , T :: qualified_type_name ()) } fn create_type_info (registry : & mut registry :: Registry) -> String { T :: create_type_info (registry) ; Self :: qualified_type_name () } async fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> ServerResult < Value > { resolve_list (ctx , field , self , Some (self . len ())) . await } }
};
}

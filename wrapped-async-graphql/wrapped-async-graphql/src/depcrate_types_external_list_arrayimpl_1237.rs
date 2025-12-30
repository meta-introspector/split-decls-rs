// Generated macro for impl_1237 (impl)
macro_rules! Depcrate_types_external_list_arrayimpl_1237 {
() => {
// Module: crate::types::external::list::array
// Provides: {"impl_1237"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : OutputType , const N : usize > OutputType for [T ; N] { fn type_name () -> Cow < 'static , str > { Cow :: Owned (format ! ("[{}]" , T :: qualified_type_name ())) } fn qualified_type_name () -> String { format ! ("[{}]!" , T :: qualified_type_name ()) } fn create_type_info (registry : & mut registry :: Registry) -> String { T :: create_type_info (registry) ; Self :: qualified_type_name () } async fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> ServerResult < Value > { resolve_list (ctx , field , self . iter () , Some (self . len ())) . await } }
};
}

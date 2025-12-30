// Generated macro for impl_1222 (impl)
macro_rules! Depcrate_types_external_json_object_hashbrown_hashmapimpl_1222 {
() => {
// Module: crate::types::external::json_object::hashbrown_hashmap
// Provides: {"impl_1222"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < K , V > OutputType for HashMap < K , V > where K : ToString + Eq + Hash + Send + Sync , V : Serialize + Send + Sync , { fn type_name () -> Cow < 'static , str > { < StdHashMap < K , V > as OutputType > :: type_name () } fn create_type_info (registry : & mut Registry) -> String { < StdHashMap < K , V > as OutputType > :: create_type_info (registry) } async fn resolve (& self , _ctx : & ContextSelectionSet < '_ > , _field : & Positioned < Field > ,) -> ServerResult < Value > { let mut map = IndexMap :: new () ; for (name , value) in self { map . insert (Name :: new (name . to_string ()) , to_value (value) . unwrap_or_default () ,) ; } Ok (Value :: Object (map)) } }
};
}

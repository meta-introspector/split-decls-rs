// Generated macro for impl_1231 (impl)
macro_rules! Depcrate_types_external_json_object_hashmapimpl_1231 {
() => {
// Module: crate::types::external::json_object::hashmap
// Provides: {"impl_1231"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < K , V , S > OutputType for HashMap < K , V , S > where K : ToString + Eq + Hash + Send + Sync , V : Serialize + Send + Sync , S : Send + Sync , { fn type_name () -> Cow < 'static , str > { Cow :: Borrowed ("JSONObject") } fn create_type_info (registry : & mut Registry) -> String { registry . create_output_type :: < Self , _ > (MetaTypeId :: Scalar , | _ | MetaType :: Scalar { name : < Self as OutputType > :: type_name () . to_string () , description : Some ("A scalar that can represent any JSON Object value." . to_string ()) , is_valid : None , visible : None , inaccessible : false , tags : Default :: default () , specified_by_url : None , directive_invocations : Default :: default () , requires_scopes : Default :: default () , }) } async fn resolve (& self , _ctx : & ContextSelectionSet < '_ > , _field : & Positioned < Field > ,) -> ServerResult < Value > { let mut map = IndexMap :: new () ; for (name , value) in self { map . insert (Name :: new (name . to_string ()) , to_value (value) . unwrap_or_default () ,) ; } Ok (Value :: Object (map)) } }
};
}

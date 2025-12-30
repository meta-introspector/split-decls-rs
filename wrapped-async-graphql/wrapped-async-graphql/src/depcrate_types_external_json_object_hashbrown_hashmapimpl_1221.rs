// Generated macro for impl_1221 (impl)
macro_rules! Depcrate_types_external_json_object_hashbrown_hashmapimpl_1221 {
() => {
// Module: crate::types::external::json_object::hashbrown_hashmap
// Provides: {"impl_1221"}
// Dependencies: {}
impl < K , V > InputType for HashMap < K , V > where K : ToString + FromStr + Eq + Hash + Send + Sync , K :: Err : Display , V : Serialize + DeserializeOwned + Send + Sync , { type RawValueType = Self ; fn type_name () -> Cow < 'static , str > { < StdHashMap < K , V > as InputType > :: type_name () } fn create_type_info (registry : & mut Registry) -> String { < StdHashMap < K , V > as InputType > :: create_type_info (registry) } fn parse (value : Option < Value >) -> InputValueResult < Self > { let value = value . unwrap_or_default () ; match value { Value :: Object (map) => map . into_iter () . map (| (name , value) | { Ok ((K :: from_str (& name) . map_err (| err | { InputValueError :: < Self > :: custom (format ! ("object key: {}" , err)) }) ? , from_value (value) . map_err (| err | format ! ("object value: {}" , err)) ? ,)) }) . collect :: < Result < _ , _ > > () . map_err (InputValueError :: propagate) , _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { let mut map = IndexMap :: new () ; for (name , value) in self { map . insert (Name :: new (name . to_string ()) , to_value (value) . unwrap_or_default () ,) ; } Value :: Object (map) } fn as_raw_value (& self) -> Option < & Self :: RawValueType > { Some (self) } }
};
}

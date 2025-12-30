// Generated macro for impl_1252 (impl)
macro_rules! Depcrate_types_external_list_hashbrown_hash_setimpl_1252 {
() => {
// Module: crate::types::external::list::hashbrown_hash_set
// Provides: {"impl_1252"}
// Dependencies: {}
impl < T : InputType + Hash + Eq > InputType for HashSet < T > { type RawValueType = Self ; fn type_name () -> Cow < 'static , str > { < StdHashSet < T > as InputType > :: type_name () } fn qualified_type_name () -> String { < StdHashSet < T > as InputType > :: qualified_type_name () } fn create_type_info (registry : & mut registry :: Registry) -> String { < StdHashSet < T > as InputType > :: create_type_info (registry) } fn parse (value : Option < Value >) -> InputValueResult < Self > { match value . unwrap_or_default () { Value :: List (values) => values . into_iter () . map (| value | InputType :: parse (Some (value))) . collect :: < Result < _ , _ > > () . map_err (InputValueError :: propagate) , value => Ok ({ let mut result = Self :: default () ; result . insert (InputType :: parse (Some (value)) . map_err (InputValueError :: propagate) ?) ; result }) , } } fn to_value (& self) -> Value { Value :: List (self . iter () . map (InputType :: to_value) . collect ()) } fn as_raw_value (& self) -> Option < & Self :: RawValueType > { Some (self) } }
};
}

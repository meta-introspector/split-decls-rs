// Generated macro for impl_1246 (impl)
macro_rules! Depcrate_types_external_list_hash_setimpl_1246 {
() => {
// Module: crate::types::external::list::hash_set
// Provides: {"impl_1246"}
// Dependencies: {}
impl < T : InputType + Hash + Eq > InputType for HashSet < T > { type RawValueType = Self ; fn type_name () -> Cow < 'static , str > { Cow :: Owned (format ! ("[{}]" , T :: qualified_type_name ())) } fn qualified_type_name () -> String { format ! ("[{}]!" , T :: qualified_type_name ()) } fn create_type_info (registry : & mut registry :: Registry) -> String { T :: create_type_info (registry) ; Self :: qualified_type_name () } fn parse (value : Option < Value >) -> InputValueResult < Self > { match value . unwrap_or_default () { Value :: List (values) => values . into_iter () . map (| value | InputType :: parse (Some (value))) . collect :: < Result < _ , _ > > () . map_err (InputValueError :: propagate) , value => Ok ({ let mut result = Self :: default () ; result . insert (InputType :: parse (Some (value)) . map_err (InputValueError :: propagate) ?) ; result }) , } } fn to_value (& self) -> Value { Value :: List (self . iter () . map (InputType :: to_value) . collect ()) } fn as_raw_value (& self) -> Option < & Self :: RawValueType > { Some (self) } }
};
}

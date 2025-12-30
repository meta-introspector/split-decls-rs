// Generated macro for impl_1295 (impl)
macro_rules! Depcrate_types_external_optionalimpl_1295 {
() => {
// Module: crate::types::external::optional
// Provides: {"impl_1295"}
// Dependencies: {}
impl < T : InputType > InputType for Option < T > { type RawValueType = T :: RawValueType ; fn type_name () -> Cow < 'static , str > { T :: type_name () } fn qualified_type_name () -> String { T :: type_name () . to_string () } fn create_type_info (registry : & mut registry :: Registry) -> String { T :: create_type_info (registry) ; T :: type_name () . to_string () } fn parse (value : Option < Value >) -> InputValueResult < Self > { match value . unwrap_or_default () { Value :: Null => Ok (None) , value => Ok (Some (T :: parse (Some (value)) . map_err (InputValueError :: propagate) ? ,)) , } } fn to_value (& self) -> Value { match self { Some (value) => value . to_value () , None => Value :: Null , } } fn as_raw_value (& self) -> Option < & Self :: RawValueType > { match self { Some (value) => value . as_raw_value () , None => None , } } }
};
}

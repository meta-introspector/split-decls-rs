// Generated macro for impl_1366 (impl)
macro_rules! Depcrate_types_external_secrecyimpl_1366 {
() => {
// Module: crate::types::external::secrecy
// Provides: {"impl_1366"}
// Dependencies: {}
impl < T : InputType + Zeroize > InputType for SecretBox < T > { type RawValueType = T :: RawValueType ; fn type_name () -> Cow < 'static , str > { T :: type_name () } fn qualified_type_name () -> String { T :: qualified_type_name () } fn create_type_info (registry : & mut registry :: Registry) -> String { T :: create_type_info (registry) } fn parse (value : Option < Value >) -> InputValueResult < Self > { T :: parse (value) . map (| value | SecretBox :: new (Box :: new (value))) . map_err (InputValueError :: propagate) } fn to_value (& self) -> Value { Value :: Null } fn as_raw_value (& self) -> Option < & Self :: RawValueType > { self . expose_secret () . as_raw_value () } }
};
}

// Generated macro for impl_1367 (impl)
macro_rules! Depcrate_types_external_secrecyimpl_1367 {
() => {
// Module: crate::types::external::secrecy
// Provides: {"impl_1367"}
// Dependencies: {}
impl InputType for SecretString { type RawValueType = str ; fn type_name () -> Cow < 'static , str > { String :: type_name () } fn qualified_type_name () -> String { String :: qualified_type_name () } fn create_type_info (registry : & mut registry :: Registry) -> String { String :: create_type_info (registry) } fn parse (value : Option < Value >) -> InputValueResult < Self > { String :: parse (value) . map (SecretString :: from) . map_err (InputValueError :: propagate) } fn to_value (& self) -> Value { Value :: Null } fn as_raw_value (& self) -> Option < & Self :: RawValueType > { Some (self . expose_secret ()) } }
};
}

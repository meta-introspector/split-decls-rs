// Generated macro for impl_23 (impl)
macro_rules! Depcrate_baseimpl_23 {
() => {
// Module: crate::base
// Provides: {"impl_23"}
// Dependencies: {}
impl < T : InputType > InputType for Arc < T > { type RawValueType = T :: RawValueType ; fn type_name () -> Cow < 'static , str > { T :: type_name () } fn create_type_info (registry : & mut Registry) -> String { T :: create_type_info (registry) } fn parse (value : Option < ConstValue >) -> InputValueResult < Self > { T :: parse (value) . map (Arc :: new) . map_err (InputValueError :: propagate) } fn to_value (& self) -> ConstValue { T :: to_value (& self) } fn as_raw_value (& self) -> Option < & Self :: RawValueType > { self . as_ref () . as_raw_value () } }
};
}

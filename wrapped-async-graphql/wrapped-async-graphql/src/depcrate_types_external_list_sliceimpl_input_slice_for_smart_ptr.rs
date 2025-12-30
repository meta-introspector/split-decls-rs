// Generated macro for impl_input_slice_for_smart_ptr (macro)
macro_rules! Depcrate_types_external_list_sliceimpl_input_slice_for_smart_ptr {
() => {
// Module: crate::types::external::list::slice
// Provides: {"impl_input_slice_for_smart_ptr"}
// Dependencies: {}
macro_rules ! impl_input_slice_for_smart_ptr { ($ ty : ty) => { impl < T : InputType > InputType for $ ty { type RawValueType = Self ; fn type_name () -> Cow <'static , str > { Cow :: Owned (format ! ("[{}]" , T :: qualified_type_name ())) } fn qualified_type_name () -> String { format ! ("[{}]!" , T :: qualified_type_name ()) } fn create_type_info (registry : & mut registry :: Registry) -> String { T :: create_type_info (registry) ; Self :: qualified_type_name () } fn parse (value : Option < Value >) -> InputValueResult < Self > { match value . unwrap_or_default () { Value :: List (values) => values . into_iter () . map (| value | InputType :: parse (Some (value))) . collect ::< Result < _ , _ >> () . map_err (InputValueError :: propagate) , value => { Ok (vec ! [InputType :: parse (Some (value)) . map_err (InputValueError :: propagate) ?,] . into () ,) } } } fn to_value (& self) -> Value { Value :: List (self . iter () . map (InputType :: to_value) . collect ()) } fn as_raw_value (& self) -> Option <& Self :: RawValueType > { Some (self) } } } ; }
};
}

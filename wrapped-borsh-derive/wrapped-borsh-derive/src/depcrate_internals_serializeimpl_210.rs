// Generated macro for impl_210 (impl)
macro_rules! Depcrate_internals_serializeimpl_210 {
() => {
// Module: crate::internals::serialize
// Provides: {"impl_210"}
// Dependencies: {}
impl FieldId { fn index (field_idx : usize) -> syn :: Result < Index > { let index = u32 :: try_from (field_idx) . map_err (| err | { syn :: Error :: new (Span :: call_site () , format ! ("up to 2^32 fields are supported {}" , err) ,) }) ? ; Ok (Index { index , span : Span :: call_site () , }) } pub fn new_struct_unnamed (field_idx : usize) -> syn :: Result < Self > { let index = Self :: index (field_idx) ? ; let result = Self :: StructUnnamed (index) ; Ok (result) } pub fn new_enum_unnamed (field_idx : usize) -> syn :: Result < Self > { let index = Self :: index (field_idx) ? ; let result = Self :: EnumUnnamed (index) ; Ok (result) } }
};
}

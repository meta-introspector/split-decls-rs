// Generated macro for transform_variant_fields (function)
macro_rules! Depcrate_internals_schema_enumstransform_variant_fields {
() => {
// Module: crate::internals::schema::enums
// Provides: {"transform_variant_fields"}
// Dependencies: {}
fn transform_variant_fields (mut input : Fields) -> Fields { match input { Fields :: Named (ref mut named) => { for field in & mut named . named { let field_attrs = field :: filter_attrs (field . attrs . drain (..)) . collect :: < Vec < _ > > () ; field . attrs = field_attrs ; } } Fields :: Unnamed (ref mut unnamed) => { for field in & mut unnamed . unnamed { let field_attrs = field :: filter_attrs (field . attrs . drain (..)) . collect :: < Vec < _ > > () ; field . attrs = field_attrs ; } } _ => { } } input }
};
}

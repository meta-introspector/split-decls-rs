// Generated macro for try_parse_fn (function)
macro_rules! Depcrate_schema_metatry_parse_fn {
() => {
// Module: crate::schema::meta
// Provides: {"try_parse_fn"}
// Dependencies: {}
fn try_parse_fn < S , T > (v : & InputValue < S >) -> Result < () , FieldError < S > > where T : FromInputValue < S > , T :: Error : IntoFieldError < S > , { T :: from_input_value (v) . map_err (T :: Error :: into_field_error) . map (drop) }
};
}

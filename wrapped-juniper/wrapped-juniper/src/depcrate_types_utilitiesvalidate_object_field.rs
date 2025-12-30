// Generated macro for validate_object_field (function)
macro_rules! Depcrate_types_utilitiesvalidate_object_field {
() => {
// Module: crate::types::utilities
// Provides: {"validate_object_field"}
// Dependencies: {}
# [doc = " Validates the specified field of a GraphQL object and returns an error message if the field is"] # [doc = " invalid."] fn validate_object_field < S > (schema : & SchemaType < S > , object_type : & TypeType < S > , object_fields : & [Argument < S >] , field_value : & InputValue < S > , field_key : & str ,) -> Option < String > where S : ScalarValue , { let field_type = object_fields . iter () . filter (| f | f . name == field_key) . map (| f | schema . make_type (& f . arg_type)) . next () ; if let Some (field_arg_type) = field_type { let error_message = validate_literal_value (schema , & field_arg_type , field_value) ; error_message . map (| m | error :: field (object_type , field_key , m)) } else { Some (error :: unknown_field (object_type , field_key)) } }
};
}

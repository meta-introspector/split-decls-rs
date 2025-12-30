// Generated macro for default_value_parser (function)
macro_rules! Depcrate_itemdefault_value_parser {
() => {
// Module: crate::item
// Provides: {"default_value_parser"}
// Dependencies: {}
fn default_value_parser (inner_type : & Type , span : Span) -> Method { let func = Ident :: new ("value_parser" , span) ; Method :: new (func , quote_spanned ! { span => clap :: value_parser ! (# inner_type) } ,) }
};
}

// Generated macro for parse_scalar_literal_by_infered_type (function)
macro_rules! Depcrate_parser_valueparse_scalar_literal_by_infered_type {
() => {
// Module: crate::parser::value
// Provides: {"parse_scalar_literal_by_infered_type"}
// Dependencies: {}
fn parse_scalar_literal_by_infered_type < S > (token : ScalarToken < '_ > , span : Span , schema : & SchemaType < S > ,) -> ParseResult < InputValue < S > > where S : ScalarValue , { let result = match token { ScalarToken :: String (_) => { if let Some (MetaType :: Scalar (s)) = schema . concrete_type_by_name ("String") { (s . parse_fn) (token) . map (InputValue :: Scalar) } else { Err (ParseError :: ExpectedScalarError ("There needs to be a String type" ,)) } } ScalarToken :: Int (_) => { if let Some (MetaType :: Scalar (s)) = schema . concrete_type_by_name ("Int") { (s . parse_fn) (token) . map (InputValue :: Scalar) } else { Err (ParseError :: ExpectedScalarError ("There needs to be an Int type" ,)) } } ScalarToken :: Float (_) => { if let Some (MetaType :: Scalar (s)) = schema . concrete_type_by_name ("Float") { (s . parse_fn) (token) . map (InputValue :: Scalar) } else { Err (ParseError :: ExpectedScalarError ("There needs to be a Float type" ,)) } } } ; result . map (| s | Spanning :: new (span , s)) . map_err (| e | Spanning :: new (span , e)) }
};
}

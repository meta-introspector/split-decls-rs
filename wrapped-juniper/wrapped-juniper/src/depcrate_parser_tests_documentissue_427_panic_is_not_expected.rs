// Generated macro for issue_427_panic_is_not_expected (function)
macro_rules! Depcrate_parser_tests_documentissue_427_panic_is_not_expected {
() => {
// Module: crate::parser::tests::document
// Provides: {"issue_427_panic_is_not_expected"}
// Dependencies: {}
# [test] fn issue_427_panic_is_not_expected () { struct QueryWithoutFloat ; # [crate :: graphql_object] impl QueryWithoutFloat { fn echo (value : String) -> String { value } } let schema = < SchemaType < DefaultScalarValue > > :: new :: < QueryWithoutFloat , EmptyMutation < () > , EmptySubscription < () > , > (& () , & () , & ()) ; let parse_result = parse_document_source (r##"{ echo(value: 123.0) }"## , & schema) ; assert_eq ! (parse_result . unwrap_err () . item , ParseError :: ExpectedScalarError ("There needs to be a Float type")) ; }
};
}

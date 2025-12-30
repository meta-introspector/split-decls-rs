// Generated macro for parse_operation_type (function)
macro_rules! Depcrate_parser_documentparse_operation_type {
() => {
// Module: crate::parser::document
// Provides: {"parse_operation_type"}
// Dependencies: {}
fn parse_operation_type (parser : & mut Parser < '_ >) -> ParseResult < OperationType > { match parser . peek () . item { Token :: Name ("query") => Ok (parser . next_token () ? . map (| _ | OperationType :: Query)) , Token :: Name ("mutation") => Ok (parser . next_token () ? . map (| _ | OperationType :: Mutation)) , Token :: Name ("subscription") => { Ok (parser . next_token () ? . map (| _ | OperationType :: Subscription)) } _ => Err (parser . next_token () ? . map (ParseError :: unexpected_token)) , } }
};
}

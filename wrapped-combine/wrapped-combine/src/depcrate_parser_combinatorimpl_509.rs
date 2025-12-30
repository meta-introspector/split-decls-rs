// Generated macro for impl_509 (impl)
macro_rules! Depcrate_parser_combinatorimpl_509 {
() => {
// Module: crate::parser::combinator
// Provides: {"impl_509"}
// Dependencies: {}
impl < Input , InputInner , P , C > Parser < Input > for InputConverter < InputInner , P , C > where Input : Stream , InputInner : Stream , P : Parser < InputInner > , for < 'c > C : Converter < 'c , Input , InputInner = InputInner > , { type Output = P :: Output ; type PartialState = P :: PartialState ; parse_mode ! (Input) ; fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , Input :: Error > where M : ParseMode , { let mut input_inner = match self . converter . convert (input) { Ok (x) => x , Err (err) => return PeekErr (err . into ()) , } ; self . parser . parse_mode (mode , & mut input_inner , state) . map_err (| err | self . converter . convert_error (input , err)) } }
};
}

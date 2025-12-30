// Generated macro for impl_485 (impl)
macro_rules! Depcrate_parser_combinatorimpl_485 {
() => {
// Module: crate::parser::combinator
// Provides: {"impl_485"}
// Dependencies: {}
impl < Input , O , P , R > Parser < Input > for Factory < P , R > where Input : Stream , P : FnMut (& mut Input) -> R , R : Parser < Input , Output = O > , { type Output = O ; type PartialState = R :: PartialState ; parse_mode ! (Input) ; fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { if mode . is_first () { self . 1 = None ; } self . parser (input) . parse_mode_impl (mode , input , state) } fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { if let Some (parser) = & mut self . 1 { parser . add_error (errors) ; } } fn add_committed_expected_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { if let Some (parser) = & mut self . 1 { parser . add_committed_expected_error (errors) ; } } }
};
}

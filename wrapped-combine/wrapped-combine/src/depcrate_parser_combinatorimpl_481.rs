// Generated macro for impl_481 (impl)
macro_rules! Depcrate_parser_combinatorimpl_481 {
() => {
// Module: crate::parser::combinator
// Provides: {"impl_481"}
// Dependencies: {}
impl < Input , O , P , R > Parser < Input > for Lazy < P > where Input : Stream , P : FnMut () -> R , R : Parser < Input , Output = O > , { type Output = O ; type PartialState = R :: PartialState ; fn parse_stream (& mut self , input : & mut Input) -> ParseResult < O , < Input as StreamOnce > :: Error > { (self . 0) () . parse_stream (input) } fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < O , < Input as StreamOnce > :: Error > { (self . 0) () . parse_lazy (input) } parse_mode ! (Input) ; fn parse_committed_mode < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { (self . 0) () . parse_mode (mode , input , state) } fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { (self . 0) () . parse_mode_impl (mode , input , state) } fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { (self . 0) () . add_error (errors) ; } fn add_committed_expected_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { (self . 0) () . add_committed_expected_error (errors) ; } }
};
}

// Generated macro for impl_442 (impl)
macro_rules! Depcrate_parser_combinatorimpl_442 {
() => {
// Module: crate::parser::combinator
// Provides: {"impl_442"}
// Dependencies: {}
impl < Input , O , P > Parser < Input > for LookAhead < P > where Input : Stream , P : Parser < Input , Output = O > , { type Output = O ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < O , < Input as StreamOnce > :: Error > { let before = input . checkpoint () ; let result = self . 0 . parse_lazy (input) ; ctry ! (input . reset (before) . committed ()) ; let (o , _input) = ctry ! (result) ; PeekOk (o) } forward_parser ! (Input , add_error add_committed_expected_error parser_count , 0) ; }
};
}

// Generated macro for impl_439 (impl)
macro_rules! Depcrate_parser_combinatorimpl_439 {
() => {
// Module: crate::parser::combinator
// Provides: {"impl_439"}
// Dependencies: {}
impl < Input , O , P > Parser < Input > for Try < P > where Input : Stream , P : Parser < Input , Output = O > , { type Output = O ; type PartialState = P :: PartialState ; # [inline] fn parse_stream (& mut self , input : & mut Input) -> ParseResult < O , < Input as StreamOnce > :: Error > { self . parse_lazy (input) } parse_mode ! (Input) ; # [inline] fn parse_committed_mode < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { self . parse_mode (mode , input , state) } # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { match self . 0 . parse_committed_mode (mode , input , state) { v @ CommitOk (_) | v @ PeekOk (_) | v @ PeekErr (_) => v , CommitErr (err) => { if input . is_partial () && err . is_unexpected_end_of_input () { CommitErr (err) } else { PeekErr (err . into ()) } } } } forward_parser ! (Input , add_error add_committed_expected_error parser_count , 0) ; }
};
}

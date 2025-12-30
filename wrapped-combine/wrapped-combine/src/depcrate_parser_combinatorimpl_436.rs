// Generated macro for impl_436 (impl)
macro_rules! Depcrate_parser_combinatorimpl_436 {
() => {
// Module: crate::parser::combinator
// Provides: {"impl_436"}
// Dependencies: {}
impl < Input , O , P > Parser < Input > for NotFollowedBy < P > where Input : Stream , P : Parser < Input , Output = O > , { type Output = () ; type PartialState = P :: PartialState ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { let checkpoint = input . checkpoint () ; let result = self . 0 . parse_mode (mode , input , state) ; ctry ! (input . reset (checkpoint) . committed ()) ; match result { CommitOk (_) | PeekOk (_) => PeekErr (Input :: Error :: empty (input . position ()) . into ()) , CommitErr (_) | PeekErr (_) => PeekOk (()) , } } # [inline] fn add_error (& mut self , _errors : & mut Tracked < < Input as StreamOnce > :: Error >) { } fn add_committed_expected_error (& mut self , _error : & mut Tracked < < Input as StreamOnce > :: Error >) { } forward_parser ! (Input , parser_count , 0) ; }
};
}

// Generated macro for impl_454 (impl)
macro_rules! Depcrate_parser_combinatorimpl_454 {
() => {
// Module: crate::parser::combinator
// Provides: {"impl_454"}
// Dependencies: {}
impl < Input , P , F , O , E > Parser < Input > for AndThen < P , F > where Input : Stream , P : Parser < Input > , F : FnMut (P :: Output) -> Result < O , E > , E : Into < < Input :: Error as ParseError < Input :: Token , Input :: Range , Input :: Position > > :: StreamError > , { type Output = O ; type PartialState = P :: PartialState ; parse_mode ! (Input) ; fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { let position = input . position () ; let checkpoint = input . checkpoint () ; match self . 0 . parse_mode (mode , input , state) { PeekOk (o) => match (self . 1) (o) { Ok (o) => PeekOk (o) , Err (err) => { let err = < Input as StreamOnce > :: Error :: from_error (position , err . into ()) ; if input . is_partial () && input_at_eof (input) { ctry ! (input . reset (checkpoint) . committed ()) ; CommitErr (err) } else { PeekErr (err . into ()) } } } , CommitOk (o) => match (self . 1) (o) { Ok (o) => CommitOk (o) , Err (err) => { if input . is_partial () && input_at_eof (input) { ctry ! (input . reset (checkpoint) . committed ()) ; } CommitErr (< Input as StreamOnce > :: Error :: from_error (position , err . into () ,)) } } , PeekErr (err) => PeekErr (err) , CommitErr (err) => CommitErr (err) , } } forward_parser ! (Input , add_error add_committed_expected_error parser_count , 0) ; }
};
}

// Generated macro for impl_451 (impl)
macro_rules! Depcrate_parser_combinatorimpl_451 {
() => {
// Module: crate::parser::combinator
// Provides: {"impl_451"}
// Dependencies: {}
impl < Input , A , B , P , F > Parser < Input > for FlatMap < P , F > where Input : Stream , P : Parser < Input , Output = A > , F : FnMut (A) -> Result < B , Input :: Error > , { type Output = B ; type PartialState = P :: PartialState ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { match self . 0 . parse_mode (mode , input , state) { PeekOk (o) => match (self . 1) (o) { Ok (x) => PeekOk (x) , Err (err) => PeekErr (err . into ()) , } , CommitOk (o) => match (self . 1) (o) { Ok (x) => CommitOk (x) , Err (err) => CommitErr (err) , } , PeekErr (err) => PeekErr (err) , CommitErr (err) => CommitErr (err) , } } forward_parser ! (Input , add_error add_committed_expected_error parser_count , 0) ; }
};
}

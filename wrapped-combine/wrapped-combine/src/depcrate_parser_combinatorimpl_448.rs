// Generated macro for impl_448 (impl)
macro_rules! Depcrate_parser_combinatorimpl_448 {
() => {
// Module: crate::parser::combinator
// Provides: {"impl_448"}
// Dependencies: {}
impl < Input , A , B , P , F > Parser < Input > for MapInput < P , F > where Input : Stream , P : Parser < Input , Output = A > , F : FnMut (A , & mut Input) -> B , { type Output = B ; type PartialState = P :: PartialState ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { match self . 0 . parse_mode (mode , input , state) { CommitOk (x) => CommitOk ((self . 1) (x , input)) , PeekOk (x) => PeekOk ((self . 1) (x , input)) , CommitErr (err) => CommitErr (err) , PeekErr (err) => PeekErr (err) , } } forward_parser ! (Input , add_error add_committed_expected_error parser_count , 0) ; }
};
}

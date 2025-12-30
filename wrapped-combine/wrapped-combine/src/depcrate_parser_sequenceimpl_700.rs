// Generated macro for impl_700 (impl)
macro_rules! Depcrate_parser_sequenceimpl_700 {
() => {
// Module: crate::parser::sequence
// Provides: {"impl_700"}
// Dependencies: {}
impl < Input , P1 , P2 > Parser < Input > for Skip < P1 , P2 > where Input : Stream , P1 : Parser < Input > , P2 : Parser < Input > , { type Output = P1 :: Output ; type PartialState = < (P1 , Ignore < P2 >) as Parser < Input > > :: PartialState ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { self . 0 . parse_mode (mode , input , state) . map (| (a , _) | a) } forward_parser ! (Input , add_error add_committed_expected_error parser_count , 0) ; }
};
}

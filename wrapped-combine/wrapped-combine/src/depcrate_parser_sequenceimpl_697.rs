// Generated macro for impl_697 (impl)
macro_rules! Depcrate_parser_sequenceimpl_697 {
() => {
// Module: crate::parser::sequence
// Provides: {"impl_697"}
// Dependencies: {}
impl < Input , P1 , P2 > Parser < Input > for With < P1 , P2 > where Input : Stream , P1 : Parser < Input > , P2 : Parser < Input > , { type Output = P2 :: Output ; type PartialState = < (Ignore < P1 > , P2) as Parser < Input > > :: PartialState ; # [inline] fn parse_lazy (& mut self , input : & mut Input ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > { self . 0 . parse_lazy (input) . map (| (_ , b) | b) } parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { self . 0 . parse_mode (mode , input , state) . map (| (_ , b) | b) } forward_parser ! (Input , add_error add_committed_expected_error parser_count , 0) ; }
};
}

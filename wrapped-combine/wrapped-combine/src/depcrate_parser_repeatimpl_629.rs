// Generated macro for impl_629 (impl)
macro_rules! Depcrate_parser_repeatimpl_629 {
() => {
// Module: crate::parser::repeat
// Provides: {"impl_629"}
// Dependencies: {}
impl < F , Input , P , S > Parser < Input > for SepBy < F , P , S > where Input : Stream , F : Extend < P :: Output > + Default , P : Parser < Input > , S : Parser < Input > , { type Output = F ; type PartialState = < Or < SepBy1 < F , P , S > , FnParser < Input , fn (& mut Input) -> StdParseResult < F , Input > > , > as Parser < Input > > :: PartialState ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < F , Input :: Error > where M : ParseMode , { sep_by1 (& mut self . parser , & mut self . separator) . or (parser (| _ | Ok ((F :: default () , Commit :: Peek (()))))) . parse_mode (mode , input , state) } fn add_committed_expected_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { self . separator . add_error (errors) } forward_parser ! (Input , add_error parser_count , parser) ; }
};
}

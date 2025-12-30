// Generated macro for impl_635 (impl)
macro_rules! Depcrate_parser_repeatimpl_635 {
() => {
// Module: crate::parser::repeat
// Provides: {"impl_635"}
// Dependencies: {}
impl < F , Input , P , S > Parser < Input > for SepEndBy < F , P , S > where Input : Stream , F : Extend < P :: Output > + Default , P : Parser < Input > , S : Parser < Input > , { type Output = F ; type PartialState = < Or < SepEndBy1 < F , P , S > , FnParser < Input , fn (& mut Input) -> StdParseResult < F , Input > > , > as Parser < Input > > :: PartialState ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , Input :: Error > where M : ParseMode , { sep_end_by1 (& mut self . parser , & mut self . separator) . or (parser (| _ | Ok ((F :: default () , Commit :: Peek (()))))) . parse_mode (mode , input , state) } fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { self . parser . add_error (errors) } }
};
}

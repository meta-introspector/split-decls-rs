// Generated macro for impl_424 (impl)
macro_rules! Depcrate_parser_choiceimpl_424 {
() => {
// Module: crate::parser::choice
// Provides: {"impl_424"}
// Dependencies: {}
impl < Input , P > Parser < Input > for Optional < P > where Input : Stream , P : Parser < Input > , { type Output = Option < P :: Output > ; type PartialState = P :: PartialState ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { let before = input . checkpoint () ; match self . 0 . parse_mode (mode , input , state) { PeekOk (x) => PeekOk (Some (x)) , CommitOk (x) => CommitOk (Some (x)) , CommitErr (err) => CommitErr (err) , PeekErr (_) => { ctry ! (input . reset (before) . committed ()) ; PeekOk (None) } } } forward_parser ! (Input , add_error parser_count , 0) ; }
};
}

// Generated macro for impl_523 (impl)
macro_rules! Depcrate_parser_errorimpl_523 {
() => {
// Module: crate::parser::error
// Provides: {"impl_523"}
// Dependencies: {}
impl < Input , P , S > Parser < Input > for Message < P , S > where Input : Stream , P : Parser < Input > , S : for < 's > ErrorInfo < 's , Input :: Token , Input :: Range > , { type Output = P :: Output ; type PartialState = P :: PartialState ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { match self . 0 . parse_mode (mode , input , state) { CommitOk (x) => CommitOk (x) , PeekOk (x) => PeekOk (x) , CommitErr (mut err) => { err . add_message (& self . 1) ; CommitErr (err) } PeekErr (err) => PeekErr (err) , } } fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { self . 0 . add_error (errors) ; errors . error . add_message (& self . 1) ; } forward_parser ! (Input , parser_count add_committed_expected_error , 0) ; }
};
}

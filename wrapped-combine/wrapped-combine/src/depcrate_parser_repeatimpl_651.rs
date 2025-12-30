// Generated macro for impl_651 (impl)
macro_rules! Depcrate_parser_repeatimpl_651 {
() => {
// Module: crate::parser::repeat
// Provides: {"impl_651"}
// Dependencies: {}
impl < F , Input , P , E > Parser < Input > for RepeatUntil < F , P , E > where Input : Stream , F : Extend < P :: Output > + Default , P : Parser < Input > , E : Parser < Input > , { type Output = F ; type PartialState = (F , bool , P :: PartialState , E :: PartialState) ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mut mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , Input :: Error > where M : ParseMode , { let (output , is_parse , parse_state , end_state) = state ; let mut committed = Commit :: Peek (()) ; loop { if * is_parse { let (token , c) = ctry ! (self . parser . parse_mode (mode , input , parse_state)) ; output . extend (Some (token)) ; committed = committed . merge (c) ; * is_parse = false ; } else { let before = input . checkpoint () ; match self . end . parse_mode (mode , input , end_state) . into () { Ok ((_ , rest)) => { ctry ! (input . reset (before) . committed ()) ; return match committed . merge (rest) { Commit :: Commit (()) => CommitOk (mem :: take (output)) , Commit :: Peek (()) => PeekOk (mem :: take (output)) , } ; } Err (Commit :: Peek (_)) => { ctry ! (input . reset (before) . committed ()) ; mode . set_first () ; * is_parse = true ; } Err (Commit :: Commit (e)) => { ctry ! (input . reset (before) . committed ()) ; return CommitErr (e . error) ; } } } } } }
};
}

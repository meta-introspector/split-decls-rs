// Generated macro for impl_647 (impl)
macro_rules! Depcrate_parser_repeatimpl_647 {
() => {
// Module: crate::parser::repeat
// Provides: {"impl_647"}
// Dependencies: {}
impl < F , Input , P > Parser < Input > for TakeUntil < F , P > where Input : Stream , F : Extend < < Input as StreamOnce > :: Token > + Default , P : Parser < Input > , { type Output = F ; type PartialState = (F , P :: PartialState) ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , Input :: Error > where M : ParseMode , { let (ref mut output , ref mut end_state) = * state ; let mut committed = Commit :: Peek (()) ; loop { let before = input . checkpoint () ; match self . end . parse_mode (mode , input , end_state) . into () { Ok ((_ , rest)) => { ctry ! (input . reset (before) . committed ()) ; return match committed . merge (rest) { Commit :: Commit (()) => CommitOk (mem :: take (output)) , Commit :: Peek (()) => PeekOk (mem :: take (output)) , } ; } Err (Commit :: Peek (_)) => { ctry ! (input . reset (before) . committed ()) ; output . extend (Some (ctry ! (uncons (input)) . 0)) ; committed = Commit :: Commit (()) ; } Err (Commit :: Commit (e)) => { ctry ! (input . reset (before) . committed ()) ; return CommitErr (e . error) ; } } ; } } }
};
}

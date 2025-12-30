// Generated macro for impl_656 (impl)
macro_rules! Depcrate_parser_repeatimpl_656 {
() => {
// Module: crate::parser::repeat
// Provides: {"impl_656"}
// Dependencies: {}
impl < Input , P , Q > Parser < Input > for Escaped < P , Q , Input :: Token > where Input : Stream , P : Parser < Input > , < Input as StreamOnce > :: Token : PartialEq , Q : Parser < Input > , { type Output = () ; type PartialState = EscapedState < P :: PartialState , Q :: PartialState > ; fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < Self :: Output , Input :: Error > { let mut committed = Commit :: Peek (()) ; loop { match self . parser . parse_lazy (input) { PeekOk (_) => { } CommitOk (_) => { committed = Commit :: Commit (()) ; } PeekErr (_) => { let checkpoint = input . checkpoint () ; match uncons (input) { CommitOk (ref c) | PeekOk (ref c) if * c == self . escape => { match self . escape_parser . parse_committed_mode (FirstMode , input , & mut Default :: default () ,) { PeekOk (_) => { } CommitOk (_) => { committed = Commit :: Commit (()) ; } CommitErr (err) => return CommitErr (err) , PeekErr (err) => { return CommitErr (err . error) ; } } } CommitErr (err) => { return CommitErr (err) ; } _ => { ctry ! (input . reset (checkpoint) . committed ()) ; return if committed . is_peek () { PeekOk (()) } else { CommitOk (()) } ; } } } CommitErr (err) => return CommitErr (err) , } } } fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { use crate :: error ; self . parser . add_error (errors) ; errors . error . add_expected (error :: Token (self . escape . clone ())) ; } }
};
}

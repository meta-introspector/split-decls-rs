// Generated macro for impl_644 (impl)
macro_rules! Depcrate_parser_repeatimpl_644 {
() => {
// Module: crate::parser::repeat
// Provides: {"impl_644"}
// Dependencies: {}
impl < Input , P , Op > Parser < Input > for Chainr1 < P , Op > where Input : Stream , P : Parser < Input > , Op : Parser < Input > , Op :: Output : FnOnce (P :: Output , P :: Output) -> P :: Output , { type Output = P :: Output ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < P :: Output , Input :: Error > { let (mut l , mut committed) = ctry ! (self . 0 . parse_lazy (input)) ; loop { let before = input . checkpoint () ; let op = match self . 1 . parse_lazy (input) . into () { Ok ((x , rest)) => { committed = committed . merge (rest) ; x } Err (Commit :: Commit (err)) => return CommitErr (err . error) , Err (Commit :: Peek (_)) => { ctry ! (input . reset (before) . committed ()) ; break ; } } ; let before = input . checkpoint () ; match self . parse_lazy (input) . into () { Ok ((r , rest)) => { l = op (l , r) ; committed = committed . merge (rest) ; } Err (Commit :: Commit (err)) => return CommitErr (err . error) , Err (Commit :: Peek (_)) => { ctry ! (input . reset (before) . committed ()) ; break ; } } } Ok ((l , committed)) . into () } fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { self . 0 . add_error (errors) } }
};
}

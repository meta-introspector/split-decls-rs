// Generated macro for impl_641 (impl)
macro_rules! Depcrate_parser_repeatimpl_641 {
() => {
// Module: crate::parser::repeat
// Provides: {"impl_641"}
// Dependencies: {}
impl < Input , P , Op > Parser < Input > for Chainl1 < P , Op > where Input : Stream , P : Parser < Input > , Op : Parser < Input > , Op :: Output : FnOnce (P :: Output , P :: Output) -> P :: Output , { type Output = P :: Output ; type PartialState = (Option < (P :: Output , Commit < () >) > , < (Op , P) as Parser < Input > > :: PartialState ,) ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mut mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , Input :: Error > where M : ParseMode , { let (ref mut l_state , ref mut child_state) = * state ; let (mut l , mut committed) = match l_state . take () { Some (x) => x , None => { let x = ctry ! (self . 0 . parse_partial (input , & mut child_state . B . state)) ; mode . set_first () ; x } } ; loop { let before = input . checkpoint () ; match (& mut self . 1 , & mut self . 0) . parse_mode (mode , input , child_state) . into () { Ok (((op , r) , rest)) => { l = op (l , r) ; committed = committed . merge (rest) ; mode . set_first () ; } Err (Commit :: Commit (err)) => { * l_state = Some ((l , committed)) ; return CommitErr (err . error) ; } Err (Commit :: Peek (_)) => { ctry ! (input . reset (before) . committed ()) ; break ; } } } Ok ((l , committed)) . into () } fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { self . 0 . add_error (errors) } }
};
}

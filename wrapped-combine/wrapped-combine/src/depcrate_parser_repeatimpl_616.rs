// Generated macro for impl_616 (impl)
macro_rules! Depcrate_parser_repeatimpl_616 {
() => {
// Module: crate::parser::repeat
// Provides: {"impl_616"}
// Dependencies: {}
impl < 'a , Input , P , S , M > Iterator for Iter < 'a , Input , P , S , M > where Input : Stream , P : Parser < Input > , S : BorrowMut < P :: PartialState > , M : ParseMode , { type Item = P :: Output ; fn next (& mut self) -> Option < P :: Output > { let before = self . input . checkpoint () ; match self . parser . parse_mode (self . mode , self . input , self . partial_state . borrow_mut ()) { PeekOk (v) => { self . mode . set_first () ; Some (v) } CommitOk (v) => { self . mode . set_first () ; self . committed = true ; Some (v) } PeekErr (e) => { self . state = match self . input . reset (before) { Err (err) => State :: CommitErr (err) , Ok (_) => State :: PeekErr (e . error) , } ; None } CommitErr (e) => { self . state = State :: CommitErr (e) ; None } } } }
};
}

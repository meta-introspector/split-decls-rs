// Generated macro for impl_80 (impl)
macro_rules! Depcrate_errorimpl_80 {
() => {
// Module: crate::error
// Provides: {"impl_80"}
// Dependencies: {}
impl < O , E > From < StdParseResult2 < O , E > > for ParseResult < O , E > { # [inline] fn from (result : StdParseResult2 < O , E >) -> ParseResult < O , E > { use self :: ParseResult :: * ; match result { Ok ((t , Commit :: Commit (()))) => CommitOk (t) , Ok ((t , Commit :: Peek (()))) => PeekOk (t) , Err (Commit :: Commit (e)) => CommitErr (e . error) , Err (Commit :: Peek (e)) => PeekErr (e) , } } }
};
}

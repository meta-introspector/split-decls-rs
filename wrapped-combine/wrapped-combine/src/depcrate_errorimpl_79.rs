// Generated macro for impl_79 (impl)
macro_rules! Depcrate_errorimpl_79 {
() => {
// Module: crate::error
// Provides: {"impl_79"}
// Dependencies: {}
impl < O , E > Into < StdParseResult2 < O , E > > for ParseResult < O , E > { # [inline] fn into (self) -> StdParseResult2 < O , E > { use self :: ParseResult :: * ; match self { CommitOk (t) => Ok ((t , Commit :: Commit (()))) , PeekOk (t) => Ok ((t , Commit :: Peek (()))) , CommitErr (e) => Err (Commit :: Commit (e . into ())) , PeekErr (e) => Err (Commit :: Peek (e)) , } } }
};
}

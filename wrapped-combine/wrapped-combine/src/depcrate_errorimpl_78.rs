// Generated macro for impl_78 (impl)
macro_rules! Depcrate_errorimpl_78 {
() => {
// Module: crate::error
// Provides: {"impl_78"}
// Dependencies: {}
impl < T , E > Into < Result < Commit < T > , Commit < Tracked < E > > > > for ParseResult < T , E > { # [inline] fn into (self) -> Result < Commit < T > , Commit < Tracked < E > > > { match self { CommitOk (t) => Ok (Commit :: Commit (t)) , PeekOk (t) => Ok (Commit :: Peek (t)) , CommitErr (e) => Err (Commit :: Commit (e . into ())) , PeekErr (e) => Err (Commit :: Peek (e)) , } } }
};
}

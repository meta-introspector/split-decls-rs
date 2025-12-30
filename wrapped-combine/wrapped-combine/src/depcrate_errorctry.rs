// Generated macro for ctry (macro)
macro_rules! Depcrate_errorctry {
() => {
// Module: crate::error
// Provides: {"ctry"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! ctry { ($ result : expr) => { match $ result { $ crate :: error :: ParseResult :: CommitOk (x) => (x , $ crate :: error :: Commit :: Commit (())) , $ crate :: error :: ParseResult :: PeekOk (x) => (x , $ crate :: error :: Commit :: Peek (())) , $ crate :: error :: ParseResult :: CommitErr (err) => { return $ crate :: error :: ParseResult :: CommitErr (err . into ()) } $ crate :: error :: ParseResult :: PeekErr (err) => { return $ crate :: error :: ParseResult :: PeekErr (err . into ()) } } } ; }
};
}

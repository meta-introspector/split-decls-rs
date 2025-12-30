// Generated macro for StdParseResult (type)
macro_rules! Depcrate_errorStdParseResult {
() => {
// Module: crate::error
// Provides: {"StdParseResult"}
// Dependencies: {}
# [doc = " A type alias over the specific `Result` type used by parsers to indicate whether they were"] # [doc = " successful or not."] # [doc = " `O` is the type that is output on success."] # [doc = " `Input` is the specific stream type used in the parser."] pub type StdParseResult < O , Input > = Result < (O , Commit < () >) , Commit < Tracked < < Input as StreamOnce > :: Error > > > ;
};
}

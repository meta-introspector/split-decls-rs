// Generated macro for XFAIL (const)
macro_rules! Depcrate_precisionXFAIL {
() => {
// Module: crate::precision
// Provides: {"XFAIL"}
// Dependencies: {}
# [doc = " Return this to skip checks on a test that currently fails but shouldn't. Takes a description"] # [doc = " of context."] const XFAIL : fn (& 'static str) -> CheckAction = CheckAction :: AssertFailure ;
};
}

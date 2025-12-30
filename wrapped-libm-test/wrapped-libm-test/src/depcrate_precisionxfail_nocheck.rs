// Generated macro for XFAIL_NOCHECK (const)
macro_rules! Depcrate_precisionXFAIL_NOCHECK {
() => {
// Module: crate::precision
// Provides: {"XFAIL_NOCHECK"}
// Dependencies: {}
# [doc = " Indicates that we expect a test to fail but we aren't asserting that it does (e.g. some results"] # [doc = " within a range do actually pass)."] # [doc = ""] # [doc = " Same as `SKIP`, just indicates we have something to eventually fix."] const XFAIL_NOCHECK : CheckAction = CheckAction :: Skip ;
};
}

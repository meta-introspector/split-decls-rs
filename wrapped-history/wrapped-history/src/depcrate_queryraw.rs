// Generated macro for Raw (struct)
macro_rules! Depcrate_queryRaw {
() => {
// Module: crate::query
// Provides: {"Raw"}
// Dependencies: {}
# [doc = " # Encoding for raw query strings."] # [doc = ""] # [doc = " The [`Raw`] wrapper allows for specifying a query string directly, bypassing the encoding. If"] # [doc = " you use this strategy, you need to take care to escape characters that are not allowed to"] # [doc = " appear in query strings yourself."] # [derive (Debug , Clone)] pub struct Raw < T > (pub T) ;
};
}

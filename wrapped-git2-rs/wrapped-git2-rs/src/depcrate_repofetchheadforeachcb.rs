// Generated macro for FetchheadForeachCb (type)
macro_rules! Depcrate_repoFetchheadForeachCb {
() => {
// Module: crate::repo
// Provides: {"FetchheadForeachCb"}
// Dependencies: {}
type FetchheadForeachCb < 'a > = dyn FnMut (& str , & [u8] , & Oid , bool) -> bool + 'a ;
};
}

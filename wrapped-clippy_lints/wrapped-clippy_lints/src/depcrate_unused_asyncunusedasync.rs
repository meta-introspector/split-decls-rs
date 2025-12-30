// Generated macro for UnusedAsync (struct)
macro_rules! Depcrate_unused_asyncUnusedAsync {
() => {
// Module: crate::unused_async
// Provides: {"UnusedAsync"}
// Dependencies: {}
# [derive (Default)] pub struct UnusedAsync { # [doc = " Keeps track of async functions used as values (i.e. path expressions to async functions that"] # [doc = " are not immediately called)"] async_fns_as_value : LocalDefIdSet , # [doc = " Functions with unused `async`, linted post-crate after we've found all uses of local async"] # [doc = " functions"] unused_async_fns : Vec < UnusedAsyncFn > , }
};
}

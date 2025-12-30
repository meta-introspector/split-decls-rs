// Generated macro for err_no_sync_resolvers (function)
macro_rules! Depcrate_graphql_object_attrerr_no_sync_resolvers {
() => {
// Module: crate::graphql_object::attr
// Provides: {"err_no_sync_resolvers"}
// Dependencies: {}
# [doc = " Emits \"synchronous resolvers are not supported\" [`syn::Error`] pointing to"] # [doc = " the given `span`."] # [must_use] fn err_no_sync_resolvers < T , S : Spanned > (span : & S) -> Option < T > { ERR . custom (span . span () , "synchronous resolvers are not supported") . note ("Specify that this function is async: `async fn foo()`") . emit () ; None }
};
}

// Generated macro for sync_resolving_code (function)
macro_rules! Depcrate_common_generatesync_resolving_code {
() => {
// Module: crate::common::generate
// Provides: {"sync_resolving_code"}
// Dependencies: {}
# [doc = " Generate the code resolving some [GraphQL type][1] in a synchronous manner."] # [doc = ""] # [doc = " Value of a [GraphQL type][1] should be stored in a `res` binding in the generated code, before"] # [doc = " including this piece of code."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Types"] pub (crate) fn sync_resolving_code () -> TokenStream { quote ! { :: juniper :: IntoResolvable :: into_resolvable (res , executor . context ()) . and_then (| res | match res { :: core :: option :: Option :: Some ((ctx , r)) => { executor . replaced_context (ctx) . resolve_with_ctx (info , & r) } :: core :: option :: Option :: None => { :: core :: result :: Result :: Ok (:: juniper :: Value :: null ()) } }) } }
};
}

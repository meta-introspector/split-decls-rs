// Generated macro for async_resolving_code (function)
macro_rules! Depcrate_common_generateasync_resolving_code {
() => {
// Module: crate::common::generate
// Provides: {"async_resolving_code"}
// Dependencies: {}
# [doc = " Generate the code resolving some [GraphQL type][1] in an asynchronous manner."] # [doc = ""] # [doc = " Value of a [GraphQL type][1] should be resolvable with `fut` binding representing a [`Future`]"] # [doc = " in the generated code, before including this piece of code."] # [doc = ""] # [doc = " Optional `ty` argument may be used to annotate a concrete type of the resolving"] # [doc = " [GraphQL type][1] (the [`Future::Output`])."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Types"] pub (crate) fn async_resolving_code (ty : Option < & syn :: Type >) -> TokenStream { let ty = ty . map (| t | quote ! { : # t }) ; quote ! { :: std :: boxed :: Box :: pin (:: juniper :: futures :: FutureExt :: then (fut , move | res # ty | async move { match :: juniper :: IntoResolvable :: into_resolvable (res , executor . context ()) ? { :: core :: option :: Option :: Some ((ctx , r)) => { let subexec = executor . replaced_context (ctx) ; subexec . resolve_with_ctx_async (info , & r) . await } :: core :: option :: Option :: None => { :: core :: result :: Result :: Ok (:: juniper :: Value :: null ()) } } })) } }
};
}

// Generated macro for expand_abort_on_panic (function)
macro_rules! Depcrateexpand_abort_on_panic {
() => {
// Module: crate
// Provides: {"expand_abort_on_panic"}
// Dependencies: {}
fn expand_abort_on_panic (mut function : ItemFn) -> TokenStream2 { let mut move_self = None ; let mut arg_pat = Vec :: new () ; let mut arg_val = Vec :: new () ; for (i , input) in function . sig . inputs . iter_mut () . enumerate () { let numbered = Ident :: new (& format ! ("__arg{}" , i) , Span :: call_site ()) ; match input { FnArg :: Typed (PatType { pat , .. }) if match pat . as_ref () { Pat :: Ident (pat) => pat . ident != "self" , _ => true , } => { arg_pat . push (quote ! (# pat)) ; arg_val . push (quote ! (# numbered)) ; * pat = parse_quote ! (mut # numbered) ; } FnArg :: Typed (_) | FnArg :: Receiver (_) => { move_self = Some (quote ! { if false { loop { } # [allow (unreachable_code)] { let __self = self ; } } }) ; } } } let ret = match & function . sig . output { ReturnType :: Default => quote ! (-> ()) , ReturnType :: Type (arrow , output) => { let mut output = output . clone () ; make_impl_trait_wild (& mut output) ; quote ! (# arrow # output) } } ; let stmts = function . block . stmts ; function . block = Box :: new (parse_quote ! ({ let __guard = :: noexcept :: __private :: AbortOnDrop ; let __result = (move || # ret { # move_self # (let # arg_pat = # arg_val ;) * # (# stmts) * }) () ; :: core :: mem :: forget (__guard) ; __result })) ; quote ! (# function) }
};
}

// Generated macro for impl_359 (impl)
macro_rules! Depcrate_graphql_unionimpl_359 {
() => {
// Module: crate::graphql_union
// Provides: {"impl_359"}
// Dependencies: {}
impl Parse for VariantAttr { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let mut out = Self :: default () ; while ! input . is_empty () { let ident = input . parse :: < syn :: Ident > () ? ; match ident . to_string () . as_str () { "ignore" | "skip" => out . ignore . replace (SpanContainer :: new (ident . span () , None , ident . clone ())) . none_or_else (| _ | err :: dup_arg (& ident)) ? , "with" => { input . parse :: < token :: Eq > () ? ; let rslvr = input . parse :: < syn :: ExprPath > () ? ; out . external_resolver . replace (SpanContainer :: new (ident . span () , Some (rslvr . span ()) , rslvr)) . none_or_else (| _ | err :: dup_arg (& ident)) ? } name => { return Err (err :: unknown_arg (& ident , name)) ; } } input . try_parse :: < token :: Comma > () ? ; } Ok (out) } }
};
}

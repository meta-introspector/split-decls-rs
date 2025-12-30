// Generated macro for impl_163 (impl)
macro_rules! Depcrate_graphql_enumimpl_163 {
() => {
// Module: crate::graphql_enum
// Provides: {"impl_163"}
// Dependencies: {}
impl Parse for VariantAttr { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let mut out = Self :: default () ; while ! input . is_empty () { let ident = input . parse_any_ident () ? ; match ident . to_string () . as_str () { "name" => { input . parse :: < token :: Eq > () ? ; let name = input . parse :: < syn :: LitStr > () ? ; out . name . replace (SpanContainer :: new (ident . span () , Some (name . span ()) , name . value () ,)) . none_or_else (| _ | err :: dup_arg (& ident)) ? } "desc" | "description" => { input . parse :: < token :: Eq > () ? ; let desc = input . parse :: < Description > () ? ; out . description . replace (SpanContainer :: new (ident . span () , Some (desc . span ()) , desc)) . none_or_else (| _ | err :: dup_arg (& ident)) ? } "deprecated" => { let directive = input . parse :: < deprecation :: Directive > () ? ; out . deprecated . replace (SpanContainer :: new (ident . span () , directive . reason . as_ref () . map (| r | r . span ()) , directive ,)) . none_or_else (| _ | err :: dup_arg (& ident)) ? } "ignore" | "skip" => out . ignore . replace (SpanContainer :: new (ident . span () , None , ident . clone ())) . none_or_else (| _ | err :: dup_arg (& ident)) ? , name => { return Err (err :: unknown_arg (& ident , name)) ; } } input . try_parse :: < token :: Comma > () ? ; } Ok (out) } }
};
}

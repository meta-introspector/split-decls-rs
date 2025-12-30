// Generated macro for impl_191 (impl)
macro_rules! Depcrate_graphql_input_objectimpl_191 {
() => {
// Module: crate::graphql_input_object
// Provides: {"impl_191"}
// Dependencies: {}
impl Parse for FieldAttr { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let mut out = Self :: default () ; while ! input . is_empty () { let ident = input . parse_any_ident () ? ; match ident . to_string () . as_str () { "name" => { input . parse :: < token :: Eq > () ? ; let name = input . parse :: < syn :: LitStr > () ? ; out . name . replace (SpanContainer :: new (ident . span () , Some (name . span ()) , name . value () ,)) . none_or_else (| _ | err :: dup_arg (& ident)) ? } "default" => { let val = input . parse :: < default :: Value > () ? ; out . default . replace (SpanContainer :: new (ident . span () , Some (val . span ()) , val)) . none_or_else (| _ | err :: dup_arg (& ident)) ? } "desc" | "description" => { input . parse :: < token :: Eq > () ? ; let desc = input . parse :: < Description > () ? ; out . description . replace (SpanContainer :: new (ident . span () , Some (desc . span ()) , desc)) . none_or_else (| _ | err :: dup_arg (& ident)) ? } "deprecated" => { let directive = input . parse :: < deprecation :: Directive > () ? ; out . deprecated . replace (SpanContainer :: new (ident . span () , directive . reason . as_ref () . map (| r | r . span ()) , directive ,)) . none_or_else (| _ | err :: dup_arg (& ident)) ? } "ignore" | "skip" => out . ignore . replace (SpanContainer :: new (ident . span () , None , ident . clone ())) . none_or_else (| _ | err :: dup_arg (& ident)) ? , name => { return Err (err :: unknown_arg (& ident , name)) ; } } input . try_parse :: < token :: Comma > () ? ; } Ok (out) } }
};
}

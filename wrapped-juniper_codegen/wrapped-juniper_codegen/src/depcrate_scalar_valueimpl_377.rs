// Generated macro for impl_377 (impl)
macro_rules! Depcrate_scalar_valueimpl_377 {
() => {
// Module: crate::scalar_value
// Provides: {"impl_377"}
// Dependencies: {}
impl Parse for Attr { fn parse (input : ParseStream < '_ >) -> syn :: Result < Attr > { let mut out = Attr :: default () ; while ! input . is_empty () { let ident = input . parse :: < syn :: Ident > () ? ; match ident . to_string () . as_str () { "from_displayable_with" => { input . parse :: < token :: Eq > () ? ; let scl = input . parse :: < syn :: ExprPath > () ? ; out . from_displayable . replace (SpanContainer :: new (ident . span () , Some (scl . span ()) , scl)) . none_or_else (| _ | err :: dup_arg (& ident)) ? } "from_displayable_non_static_with" => { input . parse :: < token :: Eq > () ? ; let scl = input . parse :: < syn :: ExprPath > () ? ; out . from_displayable_non_static . replace (SpanContainer :: new (ident . span () , Some (scl . span ()) , scl)) . none_or_else (| _ | err :: dup_arg (& ident)) ? } name => { return Err (err :: unknown_arg (& ident , name)) ; } } ; input . try_parse :: < token :: Comma > () ? ; } Ok (out) } }
};
}

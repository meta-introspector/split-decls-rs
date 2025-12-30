// Generated macro for parse_args (function)
macro_rules! Depcrate_pin_project_argsparse_args {
() => {
// Module: crate::pin_project::args
// Provides: {"parse_args"}
// Dependencies: {}
pub (super) fn parse_args (attrs : & [Attribute]) -> Result < Args > { struct Input (Option < TokenStream >) ; impl Parse for Input { fn parse (input : ParseStream < '_ >) -> Result < Self > { Ok (Self ((| | { let private = input . parse :: < Ident > () . ok () ? ; if private == "__private" { input . parenthesized () . ok () ? . parse :: < TokenStream > () . ok () } else { None } }) ())) } } if let Some (attr) = attrs . find ("pin_project") { bail ! (attr , "duplicate #[pin_project] attribute") ; } let mut attrs = attrs . iter () . filter (| attr | attr . path () . is_ident (PIN)) ; let prev = if let Some (attr) = attrs . next () { (attr , syn :: parse2 :: < Input > (attr . meta . require_list () ? . tokens . clone ()) ? . 0) } else { bail ! (TokenStream :: new () , "#[pin_project] attribute has been removed") ; } ; if let Some (attr) = attrs . next () { let (prev_attr , prev_res) = & prev ; let res = syn :: parse2 :: < Input > (attr . meta . require_list () ? . tokens . clone ()) ? . 0 ; let span = match (prev_res , res) { (Some (_) , _) => attr , (None , _) => prev_attr , } ; bail ! (span , "duplicate #[pin] attribute") ; } syn :: parse2 (prev . 1 . unwrap ()) }
};
}

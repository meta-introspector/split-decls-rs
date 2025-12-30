// Generated macro for parse_derive (function)
macro_rules! Depcrate_parse_deriveparse_derive {
() => {
// Module: crate::parse_derive
// Provides: {"parse_derive"}
// Dependencies: {}
pub (crate) fn parse_derive (ast : DeriveInput) -> (ParsedDerive , Vec < GrammarSource >) { let name = ast . ident ; let generics = ast . generics ; let grammar : Vec < & Attribute > = ast . attrs . iter () . filter (| attr | { let path = attr . meta . path () ; path . is_ident ("grammar") || path . is_ident ("grammar_inline") }) . collect () ; if grammar . is_empty () { panic ! ("a grammar file needs to be provided with the #[grammar = \"PATH\"] or #[grammar_inline = \"GRAMMAR CONTENTS\"] attribute") ; } let mut grammar_sources = Vec :: with_capacity (grammar . len ()) ; for attr in grammar { grammar_sources . push (get_attribute (attr)) } let non_exhaustive = ast . attrs . iter () . any (| attr | attr . meta . path () . is_ident ("non_exhaustive")) ; (ParsedDerive { name , generics , non_exhaustive , } , grammar_sources ,) }
};
}

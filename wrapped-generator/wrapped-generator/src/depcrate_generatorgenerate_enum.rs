// Generated macro for generate_enum (function)
macro_rules! Depcrate_generatorgenerate_enum {
() => {
// Module: crate::generator
// Provides: {"generate_enum"}
// Dependencies: {}
fn generate_enum (rules : & [OptimizedRule] , doc_comment : & DocComment , uses_eoi : bool , non_exhaustive : bool ,) -> TokenStream { let rule_variants = rules . iter () . map (| rule | { let rule_name = format_ident ! ("r#{}" , rule . name) ; match doc_comment . line_docs . get (& rule . name) { Some (doc) => quote ! { # [doc = # doc] # rule_name } , None => quote ! { # rule_name } , } }) ; let grammar_doc = & doc_comment . grammar_doc ; let mut result = if grammar_doc . is_empty () { quote ! { # [allow (dead_code , non_camel_case_types , clippy :: upper_case_acronyms)] # [derive (Clone , Copy , Debug , Eq , Hash , Ord , PartialEq , PartialOrd)] } } else { quote ! { # [doc = # grammar_doc] # [allow (dead_code , non_camel_case_types , clippy :: upper_case_acronyms)] # [derive (Clone , Copy , Debug , Eq , Hash , Ord , PartialEq , PartialOrd)] } } ; if non_exhaustive { result . append_all (quote ! { # [non_exhaustive] }) ; } result . append_all (quote ! { pub enum Rule }) ; if uses_eoi { result . append_all (quote ! { { # [doc = "End-of-input"] EOI , # (# rule_variants) ,* } }) ; } else { result . append_all (quote ! { { # (# rule_variants) ,* } }) } ; let rules = rules . iter () . map (| rule | { let rule_name = format_ident ! ("r#{}" , rule . name) ; quote ! { # rule_name } }) ; result . append_all (quote ! { impl Rule { pub fn all_rules () -> &'static [Rule] { & [# (Rule ::# rules) , *] } } }) ; result }
};
}

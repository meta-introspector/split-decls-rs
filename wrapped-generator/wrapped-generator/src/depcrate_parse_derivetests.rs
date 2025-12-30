// Generated macro for tests (module)
macro_rules! Depcrate_parse_derivetests {
() => {
// Module: crate::parse_derive
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: parse_derive ; use super :: GrammarSource ; # [test] fn derive_inline_file () { let definition = "
            #[other_attr]
            #[grammar_inline = \"GRAMMAR\"]
            pub struct MyParser<'a, T>;
        " ; let ast = syn :: parse_str (definition) . unwrap () ; let (_ , filenames) = parse_derive (ast) ; assert_eq ! (filenames , [GrammarSource :: Inline ("GRAMMAR" . to_string ())]) ; } # [test] fn derive_ok () { let definition = "
            #[other_attr]
            #[grammar = \"myfile.pest\"]
            pub struct MyParser<'a, T>;
        " ; let ast = syn :: parse_str (definition) . unwrap () ; let (parsed_derive , filenames) = parse_derive (ast) ; assert_eq ! (filenames , [GrammarSource :: File ("myfile.pest" . to_string ())]) ; assert ! (! parsed_derive . non_exhaustive) ; } # [test] fn derive_multiple_grammars () { let definition = "
            #[other_attr]
            #[grammar = \"myfile1.pest\"]
            #[grammar = \"myfile2.pest\"]
            pub struct MyParser<'a, T>;
        " ; let ast = syn :: parse_str (definition) . unwrap () ; let (_ , filenames) = parse_derive (ast) ; assert_eq ! (filenames , [GrammarSource :: File ("myfile1.pest" . to_string ()) , GrammarSource :: File ("myfile2.pest" . to_string ())]) ; } # [test] fn derive_nonexhaustive () { let definition = "
            #[non_exhaustive]
            #[grammar = \"myfile.pest\"]
            pub struct MyParser<'a, T>;
        " ; let ast = syn :: parse_str (definition) . unwrap () ; let (parsed_derive , filenames) = parse_derive (ast) ; assert_eq ! (filenames , [GrammarSource :: File ("myfile.pest" . to_string ())]) ; assert ! (parsed_derive . non_exhaustive) ; } # [test] # [should_panic (expected = "grammar attribute must be a string")] fn derive_wrong_arg () { let definition = "
            #[other_attr]
            #[grammar = 1]
            pub struct MyParser<'a, T>;
        " ; let ast = syn :: parse_str (definition) . unwrap () ; parse_derive (ast) ; } # [test] # [should_panic (expected = "a grammar file needs to be provided with the #[grammar = \"PATH\"] or #[grammar_inline = \"GRAMMAR CONTENTS\"] attribute")] fn derive_no_grammar () { let definition = "
            #[other_attr]
            pub struct MyParser<'a, T>;
        " ; let ast = syn :: parse_str (definition) . unwrap () ; parse_derive (ast) ; } }
};
}

macro_rules! deps {
    () => {
        DocComment!();
        ParsedDerive!();
    };
}

macro_rules! generate {
    () => {
        deps!();
        # [doc = " Generates the corresponding parser based based on the processed macro input. If `include_grammar`"] # [doc = " is set to true, it'll generate an explicit \"include_str\" statement (done in pest_derive, but"] # [doc = " turned off in the local bootstrap)."] pub fn generate (parsed_derive : ParsedDerive , paths : Vec < PathBuf > , rules : Vec < OptimizedRule > , defaults : Vec < & str > , doc_comment : & DocComment , include_grammar : bool ,) -> TokenStream { let uses_eoi = defaults . iter () . any (| name | * name == "EOI") ; let name = parsed_derive . name ; let builtins = generate_builtin_rules () ; let include_fix = if include_grammar { generate_include (& name , paths) } else { quote ! () } ; let rule_enum = generate_enum (& rules , doc_comment , uses_eoi , parsed_derive . non_exhaustive) ; let patterns = generate_patterns (& rules , uses_eoi) ; let skip = generate_skip (& rules) ; let mut rules : Vec < _ > = rules . into_iter () . map (generate_rule) . collect () ; rules . extend (builtins . into_iter () . filter_map (| (builtin , tokens) | { if defaults . contains (& builtin) { Some (tokens) } else { None } })) ; let (impl_generics , ty_generics , where_clause) = parsed_derive . generics . split_for_impl () ; let result = result_type () ; let parser_impl = quote ! { # [allow (clippy :: all)] impl # impl_generics :: pest :: Parser < Rule > for # name # ty_generics # where_clause { fn parse <'i > (rule : Rule , input : &'i str) -> # result < :: pest :: iterators :: Pairs <'i , Rule >, :: pest :: error :: Error < Rule > > { mod rules { #! [allow (clippy :: upper_case_acronyms)] pub mod hidden { use super :: super :: Rule ; # skip } pub mod visible { use super :: super :: Rule ; # (# rules) * } pub use self :: visible ::*; } :: pest :: state (input , | state | { match rule { # patterns } }) } } } ; quote ! { # include_fix # rule_enum # parser_impl } }
    };
}

generate!();
macro_rules! deps {
    () => {
        Rule!();
        ParserRule!();
    };
}

macro_rules! validate_ast {
    () => {
        deps!();
        # [doc = " Validates the abstract syntax tree for common mistakes:"] # [doc = " - infinite repetitions"] # [doc = " - choices that cannot be reached"] # [doc = " - left recursion"] # [allow (clippy :: ptr_arg)] pub fn validate_ast < 'a , 'i : 'a > (rules : & 'a Vec < ParserRule < 'i > >) -> Vec < Error < Rule > > { let mut errors = vec ! [] ; errors . extend (validate_repetition (rules)) ; errors . extend (validate_choices (rules)) ; errors . extend (validate_whitespace_comment (rules)) ; errors . extend (validate_left_recursion (rules)) ; # [cfg (feature = "grammar-extras")] errors . extend (validate_tag_silent_rules (rules)) ; errors . sort_by_key (| error | match error . location { InputLocation :: Span (span) => span , _ => unreachable ! () , }) ; errors }
    };
}

validate_ast!()
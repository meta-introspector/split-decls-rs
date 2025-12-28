macro_rules! deps {
    () => {
        Rule!();
    };
}

macro_rules! validate_rust_keywords {
    () => {
        deps!();
        # [doc = " Validates that the given `definitions` do not contain any Rust keywords."] # [allow (clippy :: ptr_arg)] # [deprecated = "Rust keywords are no longer restricted from the pest grammar"] pub fn validate_rust_keywords (definitions : & Vec < Span < '_ > >) -> Vec < Error < Rule > > { let mut errors = vec ! [] ; for definition in definitions { let name = definition . as_str () ; if RUST_KEYWORDS . contains (name) { errors . push (Error :: new_from_span (ErrorVariant :: CustomError { message : format ! ("{} is a rust keyword" , name) , } , * definition ,)) } } errors }
    };
}

validate_rust_keywords!()
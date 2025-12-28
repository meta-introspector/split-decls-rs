macro_rules! validate_ident_raw {
    () => {
        # [track_caller] fn validate_ident_raw (string : & str) { validate_ident (string) ; match string { "_" | "super" | "self" | "Self" | "crate" => { panic ! ("`r#{}` cannot be a raw identifier" , string) ; } _ => { } } }
    };
}

validate_ident_raw!();
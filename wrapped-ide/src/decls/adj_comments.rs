macro_rules! deps {
    () => {
        Direction!();
    };
}

macro_rules! adj_comments {
    () => {
        deps!();
        fn adj_comments (comment : & ast :: Comment , dir : Direction) -> ast :: Comment { let mut res = comment . clone () ; for element in comment . syntax () . siblings_with_tokens (dir) { let token = match element . as_token () { None => break , Some (token) => token , } ; if let Some (c) = ast :: Comment :: cast (token . clone ()) { res = c } else if token . kind () != WHITESPACE || token . text () . contains ("\n\n") { break ; } } res }
    };
}

adj_comments!()
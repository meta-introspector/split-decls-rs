macro_rules! deps {
    () => {
        PResult!();
        TokenTree!();
        Literal!();
        Ident!();
        Punct!();
        Reject!();
        Cursor!();
    };
}

macro_rules! leaf_token {
    () => {
        deps!();
        fn leaf_token (input : Cursor) -> PResult < TokenTree > { if let Ok ((input , l)) = literal (input) { Ok ((input , TokenTree :: Literal (crate :: Literal :: _new_fallback (l)))) } else if let Ok ((input , p)) = punct (input) { Ok ((input , TokenTree :: Punct (p))) } else if let Ok ((input , i)) = ident (input) { Ok ((input , TokenTree :: Ident (i))) } else if input . starts_with (ERROR) { let rest = input . advance (ERROR . len ()) ; let repr = crate :: Literal :: _new_fallback (Literal :: _new (ERROR . to_owned ())) ; Ok ((rest , TokenTree :: Literal (repr))) } else { Err (Reject) } }
    };
}

leaf_token!()
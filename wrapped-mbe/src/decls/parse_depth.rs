macro_rules! parse_depth {
    () => {
        fn parse_depth (src : & mut TtIter < '_ , Span >) -> Result < usize , () > { if src . is_empty () { Ok (0) } else if let tt :: Leaf :: Literal (tt :: Literal { symbol : text , suffix : None , .. }) = src . expect_literal () ? { text . as_str () . parse () . map_err (| _ | ()) } else { Err (()) } }
    };
}

parse_depth!()
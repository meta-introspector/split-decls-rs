macro_rules! get_expr_int {
    () => {
        fn get_expr_int (e : & Expr) -> Option < u64 > { if let Ok (Lit :: Int (ref i)) = syn :: parse2 (quote ! (# e)) { return i . base10_parse () . ok () ; } None }
    };
}

get_expr_int!();
macro_rules! get_constant_value_string {
    () => {
        fn get_constant_value_string (expr : & Box < syn :: Expr >) -> String { if let syn :: Expr :: Lit (expr_lit) = & * * expr { match & expr_lit . lit { Lit :: Int (lit_int) => lit_int . base10_digits () . to_string () , Lit :: Float (lit_float) => lit_float . base10_digits () . to_string () , _ => String :: new () , } } else { String :: new () } }
    };
}

get_constant_value_string!()
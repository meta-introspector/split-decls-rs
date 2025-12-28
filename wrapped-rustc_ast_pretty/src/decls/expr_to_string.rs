macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! expr_to_string {
    () => {
        deps!();
        pub fn expr_to_string (e : & ast :: Expr) -> String { State :: new () . expr_to_string (e) }
    };
}

expr_to_string!();
macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! stmt_to_string {
    () => {
        deps!();
        pub fn stmt_to_string (s : & ast :: Stmt) -> String { State :: new () . stmt_to_string (s) }
    };
}

stmt_to_string!()
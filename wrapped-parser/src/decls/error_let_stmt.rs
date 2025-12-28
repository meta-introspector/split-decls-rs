macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! error_let_stmt {
    () => {
        deps!();
        fn error_let_stmt (p : & mut Parser < '_ > , message : & str) { assert ! (p . at (T ! [let])) ; let m = p . start () ; p . error (message) ; expressions :: let_stmt (p , expressions :: Semicolon :: Optional) ; m . complete (p , ERROR) ; }
    };
}

error_let_stmt!()
macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! error_block {
    () => {
        deps!();
        fn error_block (p : & mut Parser < '_ > , message : & str) { assert ! (p . at (T ! ['{'])) ; let m = p . start () ; p . error (message) ; p . bump (T ! ['{']) ; expressions :: expr_block_contents (p) ; p . eat (T ! ['}']) ; m . complete (p , ERROR) ; }
    };
}

error_block!();
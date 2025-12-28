macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! abi {
    () => {
        deps!();
        fn abi (p : & mut Parser < '_ >) { assert ! (p . at (T ! [extern])) ; let abi = p . start () ; p . bump (T ! [extern]) ; p . eat (STRING) ; abi . complete (p , ABI) ; }
    };
}

abi!()
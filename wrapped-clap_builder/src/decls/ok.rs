macro_rules! ok {
    () => {
        macro_rules ! ok { ($ expr : expr) => { match $ expr { Ok (val) => val , Err (err) => { return Err (err) ; } } } ; }
    };
}

ok!()
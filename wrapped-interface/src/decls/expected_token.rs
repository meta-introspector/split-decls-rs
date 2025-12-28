macro_rules! expected_token {
    () => {
        macro_rules ! expected_token { ($ sig : tt .$ item : tt () , $ msg : expr) => { if let None = $ sig .$ item () { bail ! ($ sig , "expected {}" , $ msg) ; } } ; }
    };
}

expected_token!();
macro_rules! deps {
    () => {
        Token!();
        SsrError!();
    };
}

macro_rules! expect_token {
    () => {
        deps!();
        fn expect_token (tokens : & mut std :: vec :: IntoIter < Token > , expected : & str) -> Result < () , SsrError > { if let Some (t) = tokens . next () { if t . text == expected { return Ok (()) ; } bail ! ("Expected {} found {}" , expected , t . text) ; } bail ! ("Expected {} found end of stream" , expected) ; }
    };
}

expect_token!();
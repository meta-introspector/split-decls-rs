macro_rules! deps {
    () => {
        Literal!();
    };
}

macro_rules! assert_err {
    () => {
        deps!();
        macro_rules ! assert_err { ($ ty : ident , $ input : literal , $ kind : ident , $ ($ span : tt) +) => { assert_err_single ! ($ ty :: parse ($ input) , $ kind , $ ($ span) +) ; assert_err_single ! ($ crate :: Literal :: parse ($ input) , $ kind , $ ($ span) +) ; } ; }
    };
}

assert_err!();
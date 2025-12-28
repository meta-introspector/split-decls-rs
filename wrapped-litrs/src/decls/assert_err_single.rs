macro_rules! deps {
    () => {
        ParseErrorKind!();
    };
}

macro_rules! assert_err_single {
    () => {
        deps!();
        macro_rules ! assert_err_single { ($ expr : expr , $ kind : ident , $ ($ span : tt) +) => { let res = $ expr ; let err = match res { Err (e) => e , Ok (v) => panic ! ("Expected `{}` to return an error, but it returned Ok({:?})" , stringify ! ($ expr) , v ,) , } ; if err . kind != $ crate :: err :: ParseErrorKind ::$ kind { panic ! ("Expected error kind {} for `{}` but got {:?}" , stringify ! ($ kind) , stringify ! ($ expr) , err . kind ,) } let expected_span = assert_err_single ! (@ span $ ($ span) +) ; if err . span != expected_span { panic ! ("Expected error span {:?} for `{}` but got {:?}" , expected_span , stringify ! ($ expr) , err . span ,) } } ; (@ span $ start : literal .. $ end : literal) => { Some ($ start .. $ end) } ; (@ span $ at : literal) => { Some ($ at .. $ at + 1) } ; (@ span None) => { None } ; }
    };
}

assert_err_single!();
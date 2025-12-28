macro_rules! deps {
    () => {
        BuiltinAttribute!();
    };
}

macro_rules! rustc_attr {
    () => {
        deps!();
        macro_rules ! rustc_attr { (TEST , $ attr : ident , $ typ : expr , $ tpl : expr , $ duplicate : expr $ (, @ only_local : $ only_local : expr) ? $ (,) ?) => { rustc_attr ! ($ attr , $ typ , $ tpl , $ duplicate , $ (@ only_local : $ only_local ,) ? concat ! ("the `#[" , stringify ! ($ attr) , "]` attribute is just used for rustc unit tests \
                and will never be stable" ,) ,) } ; ($ attr : ident , $ typ : expr , $ tpl : expr , $ duplicates : expr $ (, @ only_local : $ only_local : expr) ?, $ msg : expr $ (,) ?) => { BuiltinAttribute { name : stringify ! ($ attr) , template : $ tpl } } ; }
    };
}

rustc_attr!();
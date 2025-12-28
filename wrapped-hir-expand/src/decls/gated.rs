macro_rules! deps {
    () => {
        BuiltinAttribute!();
    };
}

macro_rules! gated {
    () => {
        deps!();
        macro_rules ! gated { ($ attr : ident , $ typ : expr , $ tpl : expr , $ duplicates : expr $ (, @ only_local : $ only_local : expr) ?, $ gate : ident , $ msg : expr $ (,) ?) => { BuiltinAttribute { name : stringify ! ($ attr) , template : $ tpl } } ; ($ attr : ident , $ typ : expr , $ tpl : expr , $ duplicates : expr $ (, @ only_local : $ only_local : expr) ?, $ msg : expr $ (,) ?) => { BuiltinAttribute { name : stringify ! ($ attr) , template : $ tpl } } ; }
    };
}

gated!()
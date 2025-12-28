macro_rules! deps {
    () => {
        BuiltinAttribute!();
    };
}

macro_rules! ungated {
    () => {
        deps!();
        macro_rules ! ungated { ($ attr : ident , $ typ : expr , $ tpl : expr , $ duplicates : expr $ (, @ only_local : $ only_local : expr) ? $ (,) ?) => { BuiltinAttribute { name : stringify ! ($ attr) , template : $ tpl } } ; }
    };
}

ungated!()
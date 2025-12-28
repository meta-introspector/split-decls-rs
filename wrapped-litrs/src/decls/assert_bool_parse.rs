macro_rules! deps {
    () => {
        Literal!();
        BoolLit!();
    };
}

macro_rules! assert_bool_parse {
    () => {
        deps!();
        macro_rules ! assert_bool_parse { ($ input : literal , $ expected : expr) => { assert_parse_ok_eq ($ input , Literal :: parse ($ input) , Literal :: Bool ($ expected) , "Literal::parse" ,) ; assert_parse_ok_eq ($ input , BoolLit :: parse ($ input) , $ expected , "BoolLit::parse") ; } ; }
    };
}

assert_bool_parse!();
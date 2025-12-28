macro_rules! deps {
    () => {
        BoolLit!();
    };
}

macro_rules! parse_ok {
    () => {
        deps!();
        # [test] fn parse_ok () { assert_bool_parse ! ("false" , BoolLit :: False) ; assert_bool_parse ! ("true" , BoolLit :: True) ; }
    };
}

parse_ok!()
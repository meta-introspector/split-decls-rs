macro_rules! deps {
    () => {
        BoolLit!();
    };
}

macro_rules! as_str {
    () => {
        deps!();
        # [test] fn as_str () { assert_eq ! (BoolLit :: False . as_str () , "false") ; assert_eq ! (BoolLit :: True . as_str () , "true") ; }
    };
}

as_str!();
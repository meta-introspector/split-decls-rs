macro_rules! deps {
    () => {
        Literal!();
    };
}

macro_rules! empty {
    () => {
        deps!();
        # [test] fn empty () { assert_err ! (Literal , "" , Empty , None) ; }
    };
}

empty!();
macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! assert_diffs {
    () => {
        deps!();
        macro_rules ! assert_diffs { ([$ ($ kind : ident ($ text : literal)) ,* $ (,) ?] , $ solution : ident , $ msg : expr $ (,) ?) => { let expected = & [$ (Chunk ::$ kind ($ text)) ,*] ; assert ! (same_diffs (expected , &$ solution . diffs) , concat ! ($ msg , "\nexpected={:#?}\nactual={:#?}") , expected , $ solution . diffs ,) ; } ; }
    };
}

assert_diffs!();
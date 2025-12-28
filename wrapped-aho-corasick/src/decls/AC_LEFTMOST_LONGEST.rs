macro_rules! deps {
    () => {
        TestCollection!();
    };
}

macro_rules! AC_LEFTMOST_LONGEST {
    () => {
        deps!();
        # [doc = " Tests for Aho-Corasick's leftmost-longest match semantics."] const AC_LEFTMOST_LONGEST : TestCollection = & [BASICS , NON_OVERLAPPING , LEFTMOST , LEFTMOST_LONGEST , REGRESSION] ;
    };
}

AC_LEFTMOST_LONGEST!()
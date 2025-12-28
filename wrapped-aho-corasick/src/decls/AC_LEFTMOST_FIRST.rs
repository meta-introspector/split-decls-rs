macro_rules! deps {
    () => {
        TestCollection!();
    };
}

macro_rules! AC_LEFTMOST_FIRST {
    () => {
        deps!();
        # [doc = " Tests for Aho-Corasick's leftmost-first match semantics."] const AC_LEFTMOST_FIRST : TestCollection = & [BASICS , NON_OVERLAPPING , LEFTMOST , LEFTMOST_FIRST , REGRESSION] ;
    };
}

AC_LEFTMOST_FIRST!()
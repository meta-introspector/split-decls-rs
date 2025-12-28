macro_rules! deps {
    () => {
        TestCollection!();
    };
}

macro_rules! AC_STANDARD_NON_OVERLAPPING {
    () => {
        deps!();
        # [doc = " Tests for Aho-Corasick's standard non-overlapping match semantics."] const AC_STANDARD_NON_OVERLAPPING : TestCollection = & [BASICS , NON_OVERLAPPING , STANDARD , REGRESSION] ;
    };
}

AC_STANDARD_NON_OVERLAPPING!()
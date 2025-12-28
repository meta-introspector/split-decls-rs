macro_rules! deps {
    () => {
        TestCollection!();
    };
}

macro_rules! AC_STANDARD_OVERLAPPING {
    () => {
        deps!();
        # [doc = " Tests for Aho-Corasick's standard overlapping match semantics."] const AC_STANDARD_OVERLAPPING : TestCollection = & [BASICS , OVERLAPPING , REGRESSION] ;
    };
}

AC_STANDARD_OVERLAPPING!();
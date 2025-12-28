macro_rules! deps {
    () => {
        TestCollection!();
    };
}

macro_rules! AC_STANDARD_ANCHORED_NON_OVERLAPPING {
    () => {
        deps!();
        # [doc = " Tests for Aho-Corasick's anchored standard non-overlapping match semantics."] const AC_STANDARD_ANCHORED_NON_OVERLAPPING : TestCollection = & [ANCHORED_BASICS , ANCHORED_NON_OVERLAPPING , STANDARD_ANCHORED] ;
    };
}

AC_STANDARD_ANCHORED_NON_OVERLAPPING!();
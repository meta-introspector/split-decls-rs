macro_rules! deps {
    () => {
        TestCollection!();
    };
}

macro_rules! AC_LEFTMOST_FIRST_ANCHORED {
    () => {
        deps!();
        # [doc = " Tests for Aho-Corasick's anchored leftmost-first match semantics."] const AC_LEFTMOST_FIRST_ANCHORED : TestCollection = & [ANCHORED_BASICS , ANCHORED_NON_OVERLAPPING , ANCHORED_LEFTMOST , ANCHORED_LEFTMOST_FIRST ,] ;
    };
}

AC_LEFTMOST_FIRST_ANCHORED!()
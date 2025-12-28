macro_rules! deps {
    () => {
        TestCollection!();
    };
}

macro_rules! AC_LEFTMOST_LONGEST_ANCHORED {
    () => {
        deps!();
        # [doc = " Tests for Aho-Corasick's anchored leftmost-longest match semantics."] const AC_LEFTMOST_LONGEST_ANCHORED : TestCollection = & [ANCHORED_BASICS , ANCHORED_NON_OVERLAPPING , ANCHORED_LEFTMOST , ANCHORED_LEFTMOST_LONGEST ,] ;
    };
}

AC_LEFTMOST_LONGEST_ANCHORED!()
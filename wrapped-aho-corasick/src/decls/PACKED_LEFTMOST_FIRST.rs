macro_rules! deps {
    () => {
        TestCollection!();
    };
}

macro_rules! PACKED_LEFTMOST_FIRST {
    () => {
        deps!();
        # [doc = " Tests for leftmost-first match semantics."] const PACKED_LEFTMOST_FIRST : TestCollection = & [BASICS , LEFTMOST , LEFTMOST_FIRST , REGRESSION , TEDDY] ;
    };
}

PACKED_LEFTMOST_FIRST!();
macro_rules! deps {
    () => {
        TestCollection!();
    };
}

macro_rules! PACKED_LEFTMOST_LONGEST {
    () => {
        deps!();
        # [doc = " Tests for leftmost-longest match semantics."] const PACKED_LEFTMOST_LONGEST : TestCollection = & [BASICS , LEFTMOST , LEFTMOST_LONGEST , REGRESSION , TEDDY] ;
    };
}

PACKED_LEFTMOST_LONGEST!()
macro_rules! FIXUP_DUMMY_RANGE_END {
    () => {
        const FIXUP_DUMMY_RANGE_END : TextSize = TextSize :: new (! 0) ;
    };
}

FIXUP_DUMMY_RANGE_END!()
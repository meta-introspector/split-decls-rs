macro_rules! deps {
    () => {
        RangeListsOffset!();
        DebugRanges!();
    };
}

macro_rules! macro_778 {
    () => {
        deps!();
        define_section ! (DebugRanges , RangeListsOffset , "A writable `.debug_ranges` section.") ;
    };
}

macro_778!();
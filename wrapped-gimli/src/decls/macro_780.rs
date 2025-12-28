macro_rules! deps {
    () => {
        RangeListsOffset!();
    };
}

macro_rules! macro_780 {
    () => {
        deps!();
        define_offsets ! (RangeListOffsets : RangeListId => RangeListsOffset , "The section offsets of a series of range lists within the `.debug_ranges` or `.debug_rnglists` sections.") ;
    };
}

macro_780!();
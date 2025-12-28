macro_rules! deps {
    () => {
        LocationListsOffset!();
    };
}

macro_rules! macro_761 {
    () => {
        deps!();
        define_offsets ! (LocationListOffsets : LocationListId => LocationListsOffset , "The section offsets of a series of location lists within the `.debug_loc` or `.debug_loclists` sections.") ;
    };
}

macro_761!();
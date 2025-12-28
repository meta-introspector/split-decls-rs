macro_rules! deps {
    () => {
        DebugRngLists!();
        RangeListsOffset!();
    };
}

macro_rules! macro_779 {
    () => {
        deps!();
        define_section ! (DebugRngLists , RangeListsOffset , "A writable `.debug_rnglists` section.") ;
    };
}

macro_779!();
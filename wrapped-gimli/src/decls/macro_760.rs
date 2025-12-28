macro_rules! deps {
    () => {
        LocationListsOffset!();
        DebugLocLists!();
    };
}

macro_rules! macro_760 {
    () => {
        deps!();
        define_section ! (DebugLocLists , LocationListsOffset , "A writable `.debug_loclists` section.") ;
    };
}

macro_760!();
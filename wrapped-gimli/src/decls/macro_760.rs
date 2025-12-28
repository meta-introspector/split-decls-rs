macro_rules! deps {
    () => {
        DebugLocLists!();
        LocationListsOffset!();
    };
}

macro_rules! macro_760 {
    () => {
        deps!();
        define_section ! (DebugLocLists , LocationListsOffset , "A writable `.debug_loclists` section.") ;
    };
}

macro_760!()
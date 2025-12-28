macro_rules! deps {
    () => {
        LocationListsOffset!();
        DebugLoc!();
    };
}

macro_rules! macro_759 {
    () => {
        deps!();
        define_section ! (DebugLoc , LocationListsOffset , "A writable `.debug_loc` section.") ;
    };
}

macro_759!();
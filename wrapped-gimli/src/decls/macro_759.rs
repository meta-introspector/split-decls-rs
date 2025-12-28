macro_rules! deps {
    () => {
        DebugLoc!();
        LocationListsOffset!();
    };
}

macro_rules! macro_759 {
    () => {
        deps!();
        define_section ! (DebugLoc , LocationListsOffset , "A writable `.debug_loc` section.") ;
    };
}

macro_759!()
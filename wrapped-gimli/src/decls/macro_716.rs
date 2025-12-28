macro_rules! deps {
    () => {
        DebugAbbrevOffset!();
        DebugAbbrev!();
    };
}

macro_rules! macro_716 {
    () => {
        deps!();
        define_section ! (DebugAbbrev , DebugAbbrevOffset , "A writable `.debug_abbrev` section.") ;
    };
}

macro_716!();
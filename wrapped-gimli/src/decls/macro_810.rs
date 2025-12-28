macro_rules! deps {
    () => {
        DebugInfoOffset!();
        DebugInfo!();
    };
}

macro_rules! macro_810 {
    () => {
        deps!();
        define_section ! (DebugInfo , DebugInfoOffset , "A writable `.debug_info` section.") ;
    };
}

macro_810!();
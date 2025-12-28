macro_rules! deps {
    () => {
        DebugInfo!();
        DebugInfoOffset!();
    };
}

macro_rules! macro_810 {
    () => {
        deps!();
        define_section ! (DebugInfo , DebugInfoOffset , "A writable `.debug_info` section.") ;
    };
}

macro_810!()
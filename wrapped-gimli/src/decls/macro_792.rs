macro_rules! deps {
    () => {
        DebugStrOffset!();
        DebugStr!();
    };
}

macro_rules! macro_792 {
    () => {
        deps!();
        define_section ! (DebugStr , DebugStrOffset , "A writable `.debug_str` section.") ;
    };
}

macro_792!()
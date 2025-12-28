macro_rules! deps {
    () => {
        DebugLineStr!();
        DebugLineStrOffset!();
    };
}

macro_rules! macro_795 {
    () => {
        deps!();
        define_section ! (DebugLineStr , DebugLineStrOffset , "A writable `.debug_line_str` section.") ;
    };
}

macro_795!()
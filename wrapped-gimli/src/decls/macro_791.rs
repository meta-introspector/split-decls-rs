macro_rules! deps {
    () => {
        DebugStr!();
        DebugStrOffset!();
    };
}

macro_rules! macro_791 {
    () => {
        deps!();
        define_string_table ! (StringTable , StringId , DebugStr , DebugStrOffset , "A table of strings that will be stored in a `.debug_str` section.") ;
    };
}

macro_791!()
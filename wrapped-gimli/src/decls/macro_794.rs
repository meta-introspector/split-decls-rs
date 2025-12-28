macro_rules! deps {
    () => {
        DebugLineStrOffset!();
        DebugLineStr!();
    };
}

macro_rules! macro_794 {
    () => {
        deps!();
        define_string_table ! (LineStringTable , LineStringId , DebugLineStr , DebugLineStrOffset , "A table of strings that will be stored in a `.debug_line_str` section.") ;
    };
}

macro_794!()
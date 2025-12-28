macro_rules! deps {
    () => {
        DebugLineOffset!();
        DebugLine!();
    };
}

macro_rules! macro_755 {
    () => {
        deps!();
        define_section ! (DebugLine , DebugLineOffset , "A writable `.debug_line` section.") ;
    };
}

macro_755!()
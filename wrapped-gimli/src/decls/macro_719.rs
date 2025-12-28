macro_rules! deps {
    () => {
        DebugFrameOffset!();
        DebugFrame!();
    };
}

macro_rules! macro_719 {
    () => {
        deps!();
        define_section ! (DebugFrame , DebugFrameOffset , "A writable `.debug_frame` section.") ;
    };
}

macro_719!()
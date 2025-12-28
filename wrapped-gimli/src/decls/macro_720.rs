macro_rules! deps {
    () => {
        EhFrame!();
        EhFrameOffset!();
    };
}

macro_rules! macro_720 {
    () => {
        deps!();
        define_section ! (EhFrame , EhFrameOffset , "A writable `.eh_frame` section.") ;
    };
}

macro_720!()
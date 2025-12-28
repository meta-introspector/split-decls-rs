macro_rules! deps {
    () => {
        StyledChar!();
    };
}

macro_rules! StyledBuffer {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct StyledBuffer { lines : Vec < Vec < StyledChar > > , }
    };
}

StyledBuffer!()
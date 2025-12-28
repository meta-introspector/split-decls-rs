macro_rules! deps {
    () => {
        NonAscii!();
    };
}

macro_rules! Unicode {
    () => {
        deps!();
        pub enum Unicode { Ascii (u8) , NonAscii (NonAscii) , }
    };
}

Unicode!();
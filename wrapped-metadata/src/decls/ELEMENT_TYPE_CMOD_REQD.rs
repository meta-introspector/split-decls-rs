macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_CMOD_REQD {
    () => {
        deps!();
        pub const ELEMENT_TYPE_CMOD_REQD : CorElementType = 31u8 ;
    };
}

ELEMENT_TYPE_CMOD_REQD!()
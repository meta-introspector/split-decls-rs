macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_CMOD_OPT {
    () => {
        deps!();
        pub const ELEMENT_TYPE_CMOD_OPT : CorElementType = 32u8 ;
    };
}

ELEMENT_TYPE_CMOD_OPT!()
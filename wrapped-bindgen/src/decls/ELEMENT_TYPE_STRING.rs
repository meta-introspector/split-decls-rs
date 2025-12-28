macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_STRING {
    () => {
        deps!();
        pub const ELEMENT_TYPE_STRING : CorElementType = 14u8 ;
    };
}

ELEMENT_TYPE_STRING!();
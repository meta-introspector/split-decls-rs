macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_U8 {
    () => {
        deps!();
        pub const ELEMENT_TYPE_U8 : CorElementType = 11u8 ;
    };
}

ELEMENT_TYPE_U8!();
macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_BYREF {
    () => {
        deps!();
        pub const ELEMENT_TYPE_BYREF : CorElementType = 16u8 ;
    };
}

ELEMENT_TYPE_BYREF!()
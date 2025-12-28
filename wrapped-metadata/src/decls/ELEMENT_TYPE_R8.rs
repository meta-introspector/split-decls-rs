macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_R8 {
    () => {
        deps!();
        pub const ELEMENT_TYPE_R8 : CorElementType = 13u8 ;
    };
}

ELEMENT_TYPE_R8!();
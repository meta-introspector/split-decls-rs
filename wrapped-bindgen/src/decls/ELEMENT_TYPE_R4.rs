macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_R4 {
    () => {
        deps!();
        pub const ELEMENT_TYPE_R4 : CorElementType = 12u8 ;
    };
}

ELEMENT_TYPE_R4!();
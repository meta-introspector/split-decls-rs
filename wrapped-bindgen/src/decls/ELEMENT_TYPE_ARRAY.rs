macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_ARRAY {
    () => {
        deps!();
        pub const ELEMENT_TYPE_ARRAY : CorElementType = 20u8 ;
    };
}

ELEMENT_TYPE_ARRAY!();
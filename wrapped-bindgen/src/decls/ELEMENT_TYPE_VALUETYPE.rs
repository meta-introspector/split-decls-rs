macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_VALUETYPE {
    () => {
        deps!();
        pub const ELEMENT_TYPE_VALUETYPE : CorElementType = 17u8 ;
    };
}

ELEMENT_TYPE_VALUETYPE!();
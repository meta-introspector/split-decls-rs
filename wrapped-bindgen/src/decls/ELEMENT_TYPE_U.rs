macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_U {
    () => {
        deps!();
        pub const ELEMENT_TYPE_U : CorElementType = 25u8 ;
    };
}

ELEMENT_TYPE_U!();
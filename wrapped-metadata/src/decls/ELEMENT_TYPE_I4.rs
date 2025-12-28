macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_I4 {
    () => {
        deps!();
        pub const ELEMENT_TYPE_I4 : CorElementType = 8u8 ;
    };
}

ELEMENT_TYPE_I4!()
macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_U1 {
    () => {
        deps!();
        pub const ELEMENT_TYPE_U1 : CorElementType = 5u8 ;
    };
}

ELEMENT_TYPE_U1!()
macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_U2 {
    () => {
        deps!();
        pub const ELEMENT_TYPE_U2 : CorElementType = 7u8 ;
    };
}

ELEMENT_TYPE_U2!()
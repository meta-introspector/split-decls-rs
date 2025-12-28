macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_I1 {
    () => {
        deps!();
        pub const ELEMENT_TYPE_I1 : CorElementType = 4u8 ;
    };
}

ELEMENT_TYPE_I1!()
macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_I8 {
    () => {
        deps!();
        pub const ELEMENT_TYPE_I8 : CorElementType = 10u8 ;
    };
}

ELEMENT_TYPE_I8!()
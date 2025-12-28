macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_U4 {
    () => {
        deps!();
        pub const ELEMENT_TYPE_U4 : CorElementType = 9u8 ;
    };
}

ELEMENT_TYPE_U4!()
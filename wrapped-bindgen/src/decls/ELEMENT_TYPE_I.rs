macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_I {
    () => {
        deps!();
        pub const ELEMENT_TYPE_I : CorElementType = 24u8 ;
    };
}

ELEMENT_TYPE_I!()
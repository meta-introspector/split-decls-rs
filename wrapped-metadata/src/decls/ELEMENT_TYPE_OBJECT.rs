macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_OBJECT {
    () => {
        deps!();
        pub const ELEMENT_TYPE_OBJECT : CorElementType = 28u8 ;
    };
}

ELEMENT_TYPE_OBJECT!()
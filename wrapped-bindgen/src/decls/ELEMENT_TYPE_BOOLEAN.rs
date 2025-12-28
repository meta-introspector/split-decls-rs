macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_BOOLEAN {
    () => {
        deps!();
        pub const ELEMENT_TYPE_BOOLEAN : CorElementType = 2u8 ;
    };
}

ELEMENT_TYPE_BOOLEAN!();
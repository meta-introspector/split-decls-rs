macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_CHAR {
    () => {
        deps!();
        pub const ELEMENT_TYPE_CHAR : CorElementType = 3u8 ;
    };
}

ELEMENT_TYPE_CHAR!();
macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_SZARRAY {
    () => {
        deps!();
        pub const ELEMENT_TYPE_SZARRAY : CorElementType = 29u8 ;
    };
}

ELEMENT_TYPE_SZARRAY!()
macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_CLASS {
    () => {
        deps!();
        pub const ELEMENT_TYPE_CLASS : CorElementType = 18u8 ;
    };
}

ELEMENT_TYPE_CLASS!()
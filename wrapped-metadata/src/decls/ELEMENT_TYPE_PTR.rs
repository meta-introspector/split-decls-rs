macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_PTR {
    () => {
        deps!();
        pub const ELEMENT_TYPE_PTR : CorElementType = 15u8 ;
    };
}

ELEMENT_TYPE_PTR!()
macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_VAR {
    () => {
        deps!();
        pub const ELEMENT_TYPE_VAR : CorElementType = 19u8 ;
    };
}

ELEMENT_TYPE_VAR!()
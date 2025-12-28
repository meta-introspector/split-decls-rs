macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_GENERICINST {
    () => {
        deps!();
        pub const ELEMENT_TYPE_GENERICINST : CorElementType = 21u8 ;
    };
}

ELEMENT_TYPE_GENERICINST!();
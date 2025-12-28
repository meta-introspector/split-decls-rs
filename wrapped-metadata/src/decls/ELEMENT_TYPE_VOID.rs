macro_rules! deps {
    () => {
        CorElementType!();
    };
}

macro_rules! ELEMENT_TYPE_VOID {
    () => {
        deps!();
        pub const ELEMENT_TYPE_VOID : CorElementType = 1u8 ;
    };
}

ELEMENT_TYPE_VOID!();
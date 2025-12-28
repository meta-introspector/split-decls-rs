macro_rules! deps {
    () => {
        HRESULT!();
    };
}

macro_rules! E_UNEXPECTED {
    () => {
        deps!();
        pub const E_UNEXPECTED : HRESULT = 0x8000FFFF_u32 as _ ;
    };
}

E_UNEXPECTED!()
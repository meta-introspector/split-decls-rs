macro_rules! deps {
    () => {
        HRESULT!();
    };
}

macro_rules! S_FALSE {
    () => {
        deps!();
        pub const S_FALSE : HRESULT = 0x1_u32 as _ ;
    };
}

S_FALSE!()
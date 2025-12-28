macro_rules! deps {
    () => {
        IMAGE_DLL_CHARACTERISTICS!();
    };
}

macro_rules! IMAGE_DLLCHARACTERISTICS_NX_COMPAT {
    () => {
        deps!();
        pub const IMAGE_DLLCHARACTERISTICS_NX_COMPAT : IMAGE_DLL_CHARACTERISTICS = 256u16 ;
    };
}

IMAGE_DLLCHARACTERISTICS_NX_COMPAT!();
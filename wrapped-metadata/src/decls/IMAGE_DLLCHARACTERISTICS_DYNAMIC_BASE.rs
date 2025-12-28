macro_rules! deps {
    () => {
        IMAGE_DLL_CHARACTERISTICS!();
    };
}

macro_rules! IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE {
    () => {
        deps!();
        pub const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE : IMAGE_DLL_CHARACTERISTICS = 64u16 ;
    };
}

IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE!();
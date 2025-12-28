macro_rules! deps {
    () => {
        IMAGE_DLL_CHARACTERISTICS!();
    };
}

macro_rules! IMAGE_DLLCHARACTERISTICS_NO_SEH {
    () => {
        deps!();
        pub const IMAGE_DLLCHARACTERISTICS_NO_SEH : IMAGE_DLL_CHARACTERISTICS = 1024u16 ;
    };
}

IMAGE_DLLCHARACTERISTICS_NO_SEH!()
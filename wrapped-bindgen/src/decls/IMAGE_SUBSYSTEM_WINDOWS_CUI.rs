macro_rules! deps {
    () => {
        IMAGE_SUBSYSTEM!();
    };
}

macro_rules! IMAGE_SUBSYSTEM_WINDOWS_CUI {
    () => {
        deps!();
        pub const IMAGE_SUBSYSTEM_WINDOWS_CUI : IMAGE_SUBSYSTEM = 3u16 ;
    };
}

IMAGE_SUBSYSTEM_WINDOWS_CUI!()
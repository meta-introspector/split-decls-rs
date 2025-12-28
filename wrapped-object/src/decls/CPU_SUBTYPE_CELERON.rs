macro_rules! CPU_SUBTYPE_CELERON {
    () => {
        pub const CPU_SUBTYPE_CELERON : u32 = cpu_subtype_intel (7 , 6) ;
    };
}

CPU_SUBTYPE_CELERON!()
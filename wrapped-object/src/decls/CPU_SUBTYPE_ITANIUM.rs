macro_rules! CPU_SUBTYPE_ITANIUM {
    () => {
        pub const CPU_SUBTYPE_ITANIUM : u32 = cpu_subtype_intel (11 , 0) ;
    };
}

CPU_SUBTYPE_ITANIUM!()
macro_rules! CPU_SUBTYPE_XEON {
    () => {
        pub const CPU_SUBTYPE_XEON : u32 = cpu_subtype_intel (12 , 0) ;
    };
}

CPU_SUBTYPE_XEON!()
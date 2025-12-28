macro_rules! CPU_SUBTYPE_PENT {
    () => {
        pub const CPU_SUBTYPE_PENT : u32 = cpu_subtype_intel (5 , 0) ;
    };
}

CPU_SUBTYPE_PENT!();
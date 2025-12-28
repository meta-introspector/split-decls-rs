macro_rules! CPU_SUBTYPE_XEON_MP {
    () => {
        pub const CPU_SUBTYPE_XEON_MP : u32 = cpu_subtype_intel (12 , 1) ;
    };
}

CPU_SUBTYPE_XEON_MP!()
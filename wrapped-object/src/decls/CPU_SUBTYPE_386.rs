macro_rules! CPU_SUBTYPE_386 {
    () => {
        pub const CPU_SUBTYPE_386 : u32 = cpu_subtype_intel (3 , 0) ;
    };
}

CPU_SUBTYPE_386!()
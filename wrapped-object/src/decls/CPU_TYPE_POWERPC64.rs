macro_rules! CPU_TYPE_POWERPC64 {
    () => {
        pub const CPU_TYPE_POWERPC64 : u32 = CPU_TYPE_POWERPC | CPU_ARCH_ABI64 ;
    };
}

CPU_TYPE_POWERPC64!()
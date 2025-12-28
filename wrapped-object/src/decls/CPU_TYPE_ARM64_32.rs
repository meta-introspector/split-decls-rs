macro_rules! CPU_TYPE_ARM64_32 {
    () => {
        pub const CPU_TYPE_ARM64_32 : u32 = CPU_TYPE_ARM | CPU_ARCH_ABI64_32 ;
    };
}

CPU_TYPE_ARM64_32!()
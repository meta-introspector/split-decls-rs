macro_rules! CPU_TYPE_ARM64 {
    () => {
        pub const CPU_TYPE_ARM64 : u32 = CPU_TYPE_ARM | CPU_ARCH_ABI64 ;
    };
}

CPU_TYPE_ARM64!()
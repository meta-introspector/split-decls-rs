macro_rules! CPU_TYPE_X86_64 {
    () => {
        pub const CPU_TYPE_X86_64 : u32 = CPU_TYPE_X86 | CPU_ARCH_ABI64 ;
    };
}

CPU_TYPE_X86_64!()
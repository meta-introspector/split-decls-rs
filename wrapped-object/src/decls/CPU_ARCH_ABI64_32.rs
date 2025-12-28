macro_rules! CPU_ARCH_ABI64_32 {
    () => {
        # [doc = " ABI for 64-bit hardware with 32-bit types; LP32"] pub const CPU_ARCH_ABI64_32 : u32 = 0x0200_0000 ;
    };
}

CPU_ARCH_ABI64_32!()
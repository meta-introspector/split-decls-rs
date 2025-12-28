macro_rules! CPU_ARCH_ABI64 {
    () => {
        # [doc = " 64 bit ABI"] pub const CPU_ARCH_ABI64 : u32 = 0x0100_0000 ;
    };
}

CPU_ARCH_ABI64!()
macro_rules! CPU_ARCH_MASK {
    () => {
        # [doc = " mask for architecture bits"] pub const CPU_ARCH_MASK : u32 = 0xff00_0000 ;
    };
}

CPU_ARCH_MASK!();
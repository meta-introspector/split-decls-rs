macro_rules! SHT_ARM_EXIDX {
    () => {
        # [doc = " ARM unwind section."] pub const SHT_ARM_EXIDX : u32 = SHT_LOPROC + 1 ;
    };
}

SHT_ARM_EXIDX!();
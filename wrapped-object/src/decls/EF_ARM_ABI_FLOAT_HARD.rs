macro_rules! EF_ARM_ABI_FLOAT_HARD {
    () => {
        # [doc = " NB conflicts with EF_ARM_VFP_FLOAT"] pub const EF_ARM_ABI_FLOAT_HARD : u32 = 0x400 ;
    };
}

EF_ARM_ABI_FLOAT_HARD!();
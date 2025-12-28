macro_rules! EF_ARM_ABI_FLOAT_SOFT {
    () => {
        # [doc = " NB conflicts with EF_ARM_SOFT_FLOAT"] pub const EF_ARM_ABI_FLOAT_SOFT : u32 = 0x200 ;
    };
}

EF_ARM_ABI_FLOAT_SOFT!();
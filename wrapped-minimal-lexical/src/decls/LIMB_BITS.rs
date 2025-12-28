macro_rules! LIMB_BITS {
    () => {
        # [cfg (not (all (target_pointer_width = "64" , not (target_arch = "sparc"))))] pub const LIMB_BITS : usize = 32 ;
    };
}

LIMB_BITS!();
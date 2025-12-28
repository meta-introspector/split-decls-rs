macro_rules! Limb {
    () => {
        # [cfg (not (all (target_pointer_width = "64" , not (target_arch = "sparc"))))] pub type Limb = u32 ;
    };
}

Limb!();
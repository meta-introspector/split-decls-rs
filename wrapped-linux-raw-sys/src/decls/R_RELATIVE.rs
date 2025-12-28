macro_rules! R_RELATIVE {
    () => {
        # [cfg (target_arch = "arm")] pub const R_RELATIVE : u32 = 23 ;
    };
}

R_RELATIVE!()
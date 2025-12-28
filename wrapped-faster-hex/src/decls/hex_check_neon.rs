macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! hex_check_neon {
    () => {
        deps!();
        # [target_feature (enable = "neon")] # [cfg (target_arch = "aarch64")] pub unsafe fn hex_check_neon (src : & [u8]) -> bool { hex_check_neon_with_case (src , CheckCase :: None) }
    };
}

hex_check_neon!()
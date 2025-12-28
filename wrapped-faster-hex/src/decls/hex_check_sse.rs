macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! hex_check_sse {
    () => {
        deps!();
        # [doc = " # Safety"] # [doc = " Check if a byte slice is valid."] # [target_feature (enable = "sse4.1")] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] pub unsafe fn hex_check_sse (src : & [u8]) -> bool { hex_check_sse_with_case (src , CheckCase :: None) }
    };
}

hex_check_sse!();
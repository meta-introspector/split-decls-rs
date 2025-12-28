macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! hex_check_fallback {
    () => {
        deps!();
        # [doc = " Check if the input is valid hex bytes slice"] pub fn hex_check_fallback (src : & [u8]) -> bool { hex_check_fallback_with_case (src , CheckCase :: None) }
    };
}

hex_check_fallback!();
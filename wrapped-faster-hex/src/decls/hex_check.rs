macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! hex_check {
    () => {
        deps!();
        # [doc = " Check if the input is valid hex bytes slice"] pub fn hex_check (src : & [u8]) -> bool { hex_check_with_case (src , CheckCase :: None) }
    };
}

hex_check!();
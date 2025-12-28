macro_rules! hex_string {
    () => {
        # [cfg (not (feature = "alloc"))] pub fn hex_string < const N : usize > (src : & [u8]) -> String < N > { hex_string_custom_case (src , false) }
    };
}

hex_string!();
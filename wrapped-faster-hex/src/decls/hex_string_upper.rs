macro_rules! hex_string_upper {
    () => {
        # [cfg (not (feature = "alloc"))] pub fn hex_string_upper < const N : usize > (src : & [u8]) -> String < N > { hex_string_custom_case (src , true) }
    };
}

hex_string_upper!()
macro_rules! hex_encode_upper_fallback {
    () => {
        pub fn hex_encode_upper_fallback (src : & [u8] , dst : & mut [u8]) { hex_encode_custom_case_fallback (src , dst , true) }
    };
}

hex_encode_upper_fallback!()
macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! hex_encode_upper {
    () => {
        deps!();
        pub fn hex_encode_upper < 'a > (src : & [u8] , dst : & 'a mut [u8]) -> Result < & 'a mut str , Error > { hex_encode_custom (src , dst , true) }
    };
}

hex_encode_upper!();
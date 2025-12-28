macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! hex_encode {
    () => {
        deps!();
        # [doc = " Hex encode src into dst."] # [doc = " The length of dst must be at least src.len() * 2."] pub fn hex_encode < 'a > (src : & [u8] , dst : & 'a mut [u8]) -> Result < & 'a mut str , Error > { hex_encode_custom (src , dst , false) }
    };
}

hex_encode!()
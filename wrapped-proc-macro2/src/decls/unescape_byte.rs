macro_rules! deps {
    () => {
        Unescape!();
        EscapeError!();
    };
}

macro_rules! unescape_byte {
    () => {
        deps!();
        # [doc = " Unescape a byte literal"] # [doc = ""] # [doc = " Takes the contents of a byte literal (without quotes),"] # [doc = " and returns an unescaped byte or an error."] # [inline] pub fn unescape_byte (src : & str) -> Result < u8 , EscapeError > { < [u8] > :: unescape_single (& mut src . chars ()) }
    };
}

unescape_byte!()
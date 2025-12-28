macro_rules! deps {
    () => {
        Line!();
    };
}

macro_rules! CHAR_LF {
    () => {
        deps!();
        # [doc = " Line feed"] pub (crate) const CHAR_LF : u8 = 0x0a ;
    };
}

CHAR_LF!()
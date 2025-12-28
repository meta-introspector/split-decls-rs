macro_rules! deps {
    () => {
        Alphabet!();
    };
}

macro_rules! BIN_HEX {
    () => {
        deps!();
        # [doc = " The alphabet used in `BinHex` 4.0 files."] # [doc = ""] # [doc = " See [BinHex 4.0 Definition](http://files.stairways.com/other/binhex-40-specs-info.txt)"] pub const BIN_HEX : Alphabet = Alphabet :: from_str_unchecked ("!\"#$%&'()*+,-012345689@ABCDEFGHIJKLMNPQRSTUVXYZ[`abcdefhijklmpqr" ,) ;
    };
}

BIN_HEX!()
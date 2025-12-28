macro_rules! deps {
    () => {
        Alphabet!();
    };
}

macro_rules! CRYPT {
    () => {
        deps!();
        # [doc = " The `crypt(3)` alphabet (with `.` and `/` as the _first_ two characters)."] # [doc = ""] # [doc = " Not standardized, but folk wisdom on the net asserts that this alphabet is what crypt uses."] pub const CRYPT : Alphabet = Alphabet :: from_str_unchecked ("./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz" ,) ;
    };
}

CRYPT!();
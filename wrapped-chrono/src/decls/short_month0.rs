macro_rules! deps {
    () => {
        ParseResult!();
    };
}

macro_rules! short_month0 {
    () => {
        deps!();
        # [doc = " Tries to parse the month index (0 through 11) with the first three ASCII letters."] pub (super) fn short_month0 (s : & str) -> ParseResult < (& str , u8) > { if s . len () < 3 { return Err (TOO_SHORT) ; } let buf = s . as_bytes () ; let month0 = match (buf [0] | 32 , buf [1] | 32 , buf [2] | 32) { (b'j' , b'a' , b'n') => 0 , (b'f' , b'e' , b'b') => 1 , (b'm' , b'a' , b'r') => 2 , (b'a' , b'p' , b'r') => 3 , (b'm' , b'a' , b'y') => 4 , (b'j' , b'u' , b'n') => 5 , (b'j' , b'u' , b'l') => 6 , (b'a' , b'u' , b'g') => 7 , (b's' , b'e' , b'p') => 8 , (b'o' , b'c' , b't') => 9 , (b'n' , b'o' , b'v') => 10 , (b'd' , b'e' , b'c') => 11 , _ => return Err (INVALID) , } ; Ok ((& s [3 ..] , month0)) }
    };
}

short_month0!();
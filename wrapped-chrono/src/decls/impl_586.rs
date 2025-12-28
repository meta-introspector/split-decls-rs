macro_rules! deps {
    () => {
        Error!();
        TimeZoneName!();
        LocalTimeType!();
    };
}

macro_rules! impl_586 {
    () => {
        deps!();
        impl TimeZoneName { # [doc = " Construct a time zone name"] # [doc = ""] # [doc = " man tzfile(5):"] # [doc = " Time zone designations should consist of at least three (3) and no more than six (6) ASCII"] # [doc = " characters from the set of alphanumerics, “-”, and “+”. This is for compatibility with"] # [doc = " POSIX requirements for time zone abbreviations."] fn new (input : & [u8]) -> Result < Self , Error > { let len = input . len () ; if ! (3 ..= 7) . contains (& len) { return Err (Error :: LocalTimeType ("time zone name must have between 3 and 7 characters" ,)) ; } let mut bytes = [0 ; 8] ; bytes [0] = input . len () as u8 ; let mut i = 0 ; while i < len { let b = input [i] ; match b { b'0' ..= b'9' | b'A' ..= b'Z' | b'a' ..= b'z' | b'+' | b'-' => { } _ => return Err (Error :: LocalTimeType ("invalid characters in time zone name")) , } bytes [i + 1] = b ; i += 1 ; } Ok (Self { bytes }) } # [doc = " Returns time zone name as a byte slice"] fn as_bytes (& self) -> & [u8] { match self . bytes [0] { 3 => & self . bytes [1 .. 4] , 4 => & self . bytes [1 .. 5] , 5 => & self . bytes [1 .. 6] , 6 => & self . bytes [1 .. 7] , 7 => & self . bytes [1 .. 8] , _ => unreachable ! () , } } # [doc = " Check if two time zone names are equal"] fn equal (& self , other : & Self) -> bool { self . bytes == other . bytes } }
    };
}

impl_586!();
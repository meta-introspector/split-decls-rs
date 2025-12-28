macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! read_be_i64 {
    () => {
        deps!();
        pub (crate) fn read_be_i64 (bytes : & [u8]) -> Result < i64 , Error > { if bytes . len () != 8 { return Err (Error :: InvalidSlice ("too short for i64")) ; } let mut buf = [0 ; 8] ; buf . copy_from_slice (bytes) ; Ok (i64 :: from_be_bytes (buf)) }
    };
}

read_be_i64!();
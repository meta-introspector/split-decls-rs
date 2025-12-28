macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! read_be_i32 {
    () => {
        deps!();
        pub (crate) fn read_be_i32 (bytes : & [u8]) -> Result < i32 , Error > { if bytes . len () != 4 { return Err (Error :: InvalidSlice ("too short for i32")) ; } let mut buf = [0 ; 4] ; buf . copy_from_slice (bytes) ; Ok (i32 :: from_be_bytes (buf)) }
    };
}

read_be_i32!();
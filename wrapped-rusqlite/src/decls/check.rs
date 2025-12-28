macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! check {
    () => {
        deps!();
        pub fn check (code : c_int) -> Result < () > { if code != ffi :: SQLITE_OK { Err (error_from_sqlite_code (code , None)) } else { Ok (()) } }
    };
}

check!()
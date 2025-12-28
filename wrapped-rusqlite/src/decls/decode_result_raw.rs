macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! decode_result_raw {
    () => {
        deps!();
        pub unsafe fn decode_result_raw (db : * mut ffi :: sqlite3 , code : c_int) -> Result < () > { if code == ffi :: SQLITE_OK { Ok (()) } else { Err (error_from_handle (db , code)) } }
    };
}

decode_result_raw!()
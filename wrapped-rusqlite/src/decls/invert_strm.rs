macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! invert_strm {
    () => {
        deps!();
        # [doc = " Invert a changeset"] # [inline] pub fn invert_strm (input : & mut dyn Read , output : & mut dyn Write) -> Result < () > { let input_ref = & input ; let output_ref = & output ; check (unsafe { ffi :: sqlite3changeset_invert_strm (Some (x_input) , input_ref as * const & mut dyn Read as * mut c_void , Some (x_output) , output_ref as * const & mut dyn Write as * mut c_void ,) }) }
    };
}

invert_strm!();
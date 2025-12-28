macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! concat_strm {
    () => {
        deps!();
        # [doc = " Combine two changesets"] # [inline] pub fn concat_strm (input_a : & mut dyn Read , input_b : & mut dyn Read , output : & mut dyn Write ,) -> Result < () > { let input_a_ref = & input_a ; let input_b_ref = & input_b ; let output_ref = & output ; check (unsafe { ffi :: sqlite3changeset_concat_strm (Some (x_input) , input_a_ref as * const & mut dyn Read as * mut c_void , Some (x_input) , input_b_ref as * const & mut dyn Read as * mut c_void , Some (x_output) , output_ref as * const & mut dyn Write as * mut c_void ,) }) }
    };
}

concat_strm!()
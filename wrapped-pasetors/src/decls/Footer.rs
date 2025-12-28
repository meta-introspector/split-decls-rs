macro_rules! Footer {
    () => {
        # [derive (Debug , PartialEq , Eq , Clone)] # [doc = " A footer with optional claims that are JSON-encoded."] pub struct Footer { list_of : HashMap < String , Value > , max_keys : usize , max_len : usize , }
    };
}

Footer!()
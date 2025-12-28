macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! validate_paserk_string {
    () => {
        deps!();
        # [doc = " Validate an input string to check if it is a well-formatted PASERK."] # [doc = ""] # [doc = " Return the base64-encoded part of the serialized string."] fn validate_paserk_string (input : & str , version_id : & str , type_id : & str , expected_len : usize ,) -> Result < Vec < u8 > , Error > { let split = input . split ('.') . collect :: < Vec < & str > > () ; if split . len () != 3 { return Err (Error :: PaserkParsing) ; } if split [0] == version_id && split [1] == type_id { let ret = decode_b64 (split [2]) ? ; if ret . len () != expected_len { return Err (Error :: PaserkParsing) ; } Ok (ret) } else { Err (Error :: PaserkParsing) } }
    };
}

validate_paserk_string!();
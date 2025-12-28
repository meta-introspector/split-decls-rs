macro_rules! deps {
    () => {
        Base64!();
        Error!();
    };
}

macro_rules! test_base64_invalid_padding {
    () => {
        deps!();
        # [test] fn test_base64_invalid_padding () { let valid_padding = "AA==" ; assert_eq ! (Base64 :: decode_to_vec (valid_padding , None) , Ok (vec ! [0u8 ; 1])) ; let invalid_padding = "AA=" ; assert_eq ! (Base64 :: decode_to_vec (invalid_padding , None) , Err (Error :: InvalidInput)) ; }
    };
}

test_base64_invalid_padding!();
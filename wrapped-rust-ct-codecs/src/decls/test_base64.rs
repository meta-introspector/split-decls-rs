macro_rules! deps {
    () => {
        Base64!();
    };
}

macro_rules! test_base64 {
    () => {
        deps!();
        # [cfg (feature = "std")] # [test] fn test_base64 () { let bin = [1u8 , 5 , 11 , 15 , 19 , 131 , 122] ; let expected = "AQULDxODeg==" ; let b64 = Base64 :: encode_to_string (bin) . unwrap () ; assert_eq ! (b64 , expected) ; let bin2 = Base64 :: decode_to_vec (& b64 , None) . unwrap () ; assert_eq ! (bin , & bin2 [..]) ; }
    };
}

test_base64!();
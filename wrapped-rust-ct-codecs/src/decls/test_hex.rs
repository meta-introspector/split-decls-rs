macro_rules! deps {
    () => {
        Hex!();
    };
}

macro_rules! test_hex {
    () => {
        deps!();
        # [cfg (feature = "std")] # [test] fn test_hex () { let bin = [1u8 , 5 , 11 , 15 , 19 , 131] ; let hex = Hex :: encode_to_string (bin) . unwrap () ; let expected = "01050b0f1383" ; assert_eq ! (hex , expected) ; let bin2 = Hex :: decode_to_vec (& hex , None) . unwrap () ; assert_eq ! (bin , & bin2 [..]) ; }
    };
}

test_hex!()
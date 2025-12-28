macro_rules! deps {
    () => {
        Hex!();
    };
}

macro_rules! test_hex_no_std {
    () => {
        deps!();
        # [test] fn test_hex_no_std () { let bin = [1u8 , 5 , 11 , 15 , 19 , 131] ; let expected = "01050b0f1383" ; let mut hex = [0u8 ; 12] ; let hex = Hex :: encode_to_str (& mut hex , bin) . unwrap () ; assert_eq ! (& hex , & expected) ; let mut bin2 = [0u8 ; 6] ; let bin2 = Hex :: decode (& mut bin2 , hex , None) . unwrap () ; assert_eq ! (bin , bin2) ; }
    };
}

test_hex_no_std!();
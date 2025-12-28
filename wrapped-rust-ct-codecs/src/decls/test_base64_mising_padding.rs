macro_rules! deps {
    () => {
        Base64!();
        Base64NoPadding!();
    };
}

macro_rules! test_base64_mising_padding {
    () => {
        deps!();
        # [cfg (feature = "std")] # [test] fn test_base64_mising_padding () { let missing_padding = "AA" ; assert ! (Base64 :: decode_to_vec (missing_padding , None) . is_err ()) ; assert ! (Base64NoPadding :: decode_to_vec (missing_padding , None) . is_ok ()) ; let missing_padding = "AAA" ; assert ! (Base64 :: decode_to_vec (missing_padding , None) . is_err ()) ; assert ! (Base64NoPadding :: decode_to_vec (missing_padding , None) . is_ok ()) ; }
    };
}

test_base64_mising_padding!()
macro_rules! test_repr {
    () => {
        # [test] fn test_repr () { unsafe { assert_eq ! (raw_byte_repr (& 17u8) , & [17]) ; assert_eq ! (raw_byte_repr ("abc") , "abc" . as_bytes ()) ; } }
    };
}

test_repr!()
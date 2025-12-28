macro_rules! deps {
    () => {
        CStrLike!();
        Error!();
    };
}

macro_rules! test_c_str_like_bake {
    () => {
        deps!();
        # [test] fn test_c_str_like_bake () { fn test < S : CStrLike > (value : S) -> Result < usize , S :: Error > { value . bake () . map (| value | unsafe { libc :: strlen (value . as_ptr ()) }) } assert_eq ! (Ok (3) , test ("foo")) ; assert_eq ! (Ok (3) , test (& String :: from ("foo"))) ; assert_eq ! (Ok (3) , test (CString :: new ("foo") . unwrap () . as_ref ())) ; assert_eq ! (Ok (3) , test (& CString :: new ("foo") . unwrap ())) ; assert_eq ! (Ok (3) , test (CString :: new ("foo") . unwrap ())) ; assert_eq ! (3 , test ("foo\0bar") . err () . unwrap () . nul_position ()) ; }
    };
}

test_c_str_like_bake!();
macro_rules! unit_tests {
    () => {
        # [cfg (test)] mod unit_tests { use super :: * ; # [test] fn test_le64 () { assert_eq ! (vec ! [0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , le64 (0)) ; assert_eq ! (vec ! [10 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , le64 (10)) ; } # [test] fn test_pae () { assert_eq ! ("0000000000000000" , hex :: encode (pae (& []) . unwrap ())) ; assert_eq ! ("01000000000000000000000000000000" , hex :: encode (pae (& [b""]) . unwrap ())) ; assert_eq ! ("020000000000000000000000000000000000000000000000" , hex :: encode (pae (& [b"" , b""]) . unwrap ())) ; assert_eq ! ("0100000000000000070000000000000050617261676f6e" , hex :: encode (pae (& [b"Paragon"]) . unwrap ())) ; assert_eq ! ("0200000000000000070000000000000050617261676f6e0a00000000000000496e6974696174697665" , hex :: encode (pae (& [b"Paragon" , b"Initiative" ,]) . unwrap ())) ; assert_eq ! ("0100000000000000190000000000000050617261676f6e0a00000000000000496e6974696174697665" , hex :: encode (pae (& [b"Paragon\n\0\0\0\0\0\0\0Initiative"]) . unwrap ())) ; } }
    };
}

unit_tests!()
macro_rules! deps {
    () => {
        Error!();
        Verify!();
        Err!();
        ErrorKind!();
    };
}

macro_rules! test_verify_alloc {
    () => {
        deps!();
        # [test] # [cfg (feature = "alloc")] fn test_verify_alloc () { use crate :: bytes :: complete :: take ; let mut parser1 = verify (map (take (3u8) , | s : & [u8] | s . to_vec ()) , | s : & [u8] | { s == & b"abc" [..] }) ; assert_eq ! (parser1 . parse (& b"abcd" [..]) , Ok ((& b"d" [..] , b"abc" . to_vec ()))) ; assert_eq ! (parser1 . parse (& b"defg" [..]) , Err (Err :: Error ((& b"defg" [..] , ErrorKind :: Verify)))) ; }
    };
}

test_verify_alloc!();
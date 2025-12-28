macro_rules! deps {
    () => {
        Verify!();
        IResult!();
        Needed!();
        Error!();
        Err!();
        ErrorKind!();
    };
}

macro_rules! verify_test {
    () => {
        deps!();
        # [test] fn verify_test () { use crate :: bytes :: streaming :: take ; fn test (i : & [u8]) -> IResult < & [u8] , & [u8] > { verify (take (5u8) , | slice : & [u8] | slice [0] == b'a') . parse (i) } assert_eq ! (test (& b"bcd" [..]) , Err (Err :: Incomplete (Needed :: new (2)))) ; assert_eq ! (test (& b"bcdefg" [..]) , Err (Err :: Error (error_position ! (& b"bcdefg" [..] , ErrorKind :: Verify)))) ; assert_eq ! (test (& b"abcdefg" [..]) , Ok ((& b"fg" [..] , & b"abcde" [..]))) ; }
    };
}

verify_test!()
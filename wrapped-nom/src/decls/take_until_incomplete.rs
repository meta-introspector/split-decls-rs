macro_rules! deps {
    () => {
        Err!();
        IResult!();
        Needed!();
    };
}

macro_rules! take_until_incomplete {
    () => {
        deps!();
        # [test] fn take_until_incomplete () { use crate :: bytes :: streaming :: take_until ; fn y (i : & [u8]) -> IResult < & [u8] , & [u8] > { take_until ("end") (i) } assert_eq ! (y (& b"nd" [..]) , Err (Err :: Incomplete (Needed :: Unknown))) ; assert_eq ! (y (& b"123" [..]) , Err (Err :: Incomplete (Needed :: Unknown))) ; assert_eq ! (y (& b"123en" [..]) , Err (Err :: Incomplete (Needed :: Unknown))) ; }
    };
}

take_until_incomplete!()
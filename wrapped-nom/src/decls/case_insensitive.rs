macro_rules! deps {
    () => {
        ErrorKind!();
        Err!();
        Tag!();
        Error!();
        IResult!();
        Needed!();
    };
}

macro_rules! case_insensitive {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [test] fn case_insensitive () { use crate :: bytes :: streaming :: tag_no_case ; fn test (i : & [u8]) -> IResult < & [u8] , & [u8] > { tag_no_case ("ABcd") (i) } assert_eq ! (test (& b"aBCdefgh" [..]) , Ok ((& b"efgh" [..] , & b"aBCd" [..]))) ; assert_eq ! (test (& b"abcdefgh" [..]) , Ok ((& b"efgh" [..] , & b"abcd" [..]))) ; assert_eq ! (test (& b"ABCDefgh" [..]) , Ok ((& b"efgh" [..] , & b"ABCD" [..]))) ; assert_eq ! (test (& b"ab" [..]) , Err (Err :: Incomplete (Needed :: new (2)))) ; assert_eq ! (test (& b"Hello" [..]) , Err (Err :: Error (error_position ! (& b"Hello" [..] , ErrorKind :: Tag)))) ; assert_eq ! (test (& b"Hel" [..]) , Err (Err :: Error (error_position ! (& b"Hel" [..] , ErrorKind :: Tag)))) ; fn test2 (i : & str) -> IResult < & str , & str > { tag_no_case ("ABcd") (i) } assert_eq ! (test2 ("aBCdefgh") , Ok (("efgh" , "aBCd"))) ; assert_eq ! (test2 ("abcdefgh") , Ok (("efgh" , "abcd"))) ; assert_eq ! (test2 ("ABCDefgh") , Ok (("efgh" , "ABCD"))) ; assert_eq ! (test2 ("ab") , Err (Err :: Incomplete (Needed :: new (2)))) ; assert_eq ! (test2 ("Hello") , Err (Err :: Error (error_position ! ("Hello" , ErrorKind :: Tag)))) ; assert_eq ! (test2 ("Hel") , Err (Err :: Error (error_position ! ("Hel" , ErrorKind :: Tag)))) ; }
    };
}

case_insensitive!()
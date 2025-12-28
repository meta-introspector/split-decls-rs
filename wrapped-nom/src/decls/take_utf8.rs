macro_rules! deps {
    () => {
        IResult!();
        Err!();
        Needed!();
    };
}

macro_rules! take_utf8 {
    () => {
        deps!();
        # [test] fn take_utf8 () { use crate :: bytes :: streaming :: { take , take_while } ; fn f (i : & str) -> IResult < & str , & str > { take (3_usize) (i) } assert_eq ! (f ("") , Err (Err :: Incomplete (Needed :: Unknown))) ; assert_eq ! (f ("ab") , Err (Err :: Incomplete (Needed :: Unknown))) ; assert_eq ! (f ("點") , Err (Err :: Incomplete (Needed :: Unknown))) ; assert_eq ! (f ("ab點cd") , Ok (("cd" , "ab點"))) ; assert_eq ! (f ("a點bcd") , Ok (("cd" , "a點b"))) ; assert_eq ! (f ("a點b") , Ok (("" , "a點b"))) ; fn g (i : & str) -> IResult < & str , & str > { take_while (| c | c == '點') (i) } assert_eq ! (g ("") , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (g ("點abcd") , Ok (("abcd" , "點"))) ; assert_eq ! (g ("點點點a") , Ok (("a" , "點點點"))) ; }
    };
}

take_utf8!()
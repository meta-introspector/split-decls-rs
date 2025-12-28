macro_rules! deps {
    () => {
        Escaped!();
        Error!();
        ErrorKind!();
        IResult!();
        Err!();
    };
}

macro_rules! escaping_str {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [test] fn escaping_str () { use crate :: character :: streaming :: one_of ; fn esc (i : & str) -> IResult < & str , & str > { escaped (alpha , '\\' , one_of ("\"n\\")) (i) } assert_eq ! (esc ("abcd;") , Ok ((";" , "abcd"))) ; assert_eq ! (esc ("ab\\\"cd;") , Ok ((";" , "ab\\\"cd"))) ; assert_eq ! (esc ("\\\"abcd;") , Ok ((";" , "\\\"abcd"))) ; assert_eq ! (esc ("\\n;") , Ok ((";" , "\\n"))) ; assert_eq ! (esc ("ab\\\"12") , Ok (("12" , "ab\\\""))) ; assert_eq ! (esc ("AB\\") , Err (Err :: Error (error_position ! ("AB\\" , ErrorKind :: Escaped)))) ; assert_eq ! (esc ("AB\\A") , Err (Err :: Error (error_node_position ! ("AB\\A" , ErrorKind :: Escaped , error_position ! ("A" , ErrorKind :: OneOf))))) ; fn esc2 (i : & str) -> IResult < & str , & str > { escaped (digit , '\\' , one_of ("\"n\\")) (i) } assert_eq ! (esc2 ("12\\nnn34") , Ok (("nn34" , "12\\n"))) ; fn esc3 (i : & str) -> IResult < & str , & str > { escaped (alpha , '\u{241b}' , one_of ("\"n")) (i) } assert_eq ! (esc3 ("ab␛ncd;") , Ok ((";" , "ab␛ncd"))) ; }
    };
}

escaping_str!()
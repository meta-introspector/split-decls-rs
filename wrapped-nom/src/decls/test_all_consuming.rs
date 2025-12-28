macro_rules! deps {
    () => {
        Err!();
        ErrorKind!();
        Error!();
    };
}

macro_rules! test_all_consuming {
    () => {
        deps!();
        # [test] fn test_all_consuming () { let input : & [u8] = & [100 , 101 , 102] [..] ; assert_parse ! (all_consuming (take (2usize)) . parse (input) , Err (Err :: Error ((& [102] [..] , ErrorKind :: Eof)))) ; assert_parse ! (all_consuming (take (3usize)) . parse (input) , Ok ((& [] [..] , & [100 , 101 , 102] [..]))) ; }
    };
}

test_all_consuming!()
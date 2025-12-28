macro_rules! deps {
    () => {
        Error!();
        MapOpt!();
        ErrorKind!();
        Err!();
    };
}

macro_rules! test_map_opt {
    () => {
        deps!();
        # [test] fn test_map_opt () { let input : & [u8] = & [50] [..] ; assert_parse ! (map_opt (u8 , | u | if u < 20 { Some (u) } else { None }) . parse (input) , Err (Err :: Error ((& [50] [..] , ErrorKind :: MapOpt)))) ; assert_parse ! (map_opt (u8 , | u | if u > 20 { Some (u) } else { None }) . parse (input) , Ok ((& [] [..] , 50))) ; }
    };
}

test_map_opt!();
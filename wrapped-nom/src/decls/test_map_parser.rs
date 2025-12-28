macro_rules! test_map_parser {
    () => {
        # [test] fn test_map_parser () { let input : & [u8] = & [100 , 101 , 102 , 103 , 104] [..] ; assert_parse ! (map_parser (take (4usize) , take (2usize)) . parse (input) , Ok ((& [104] [..] , & [100 , 101] [..]))) ; }
    };
}

test_map_parser!()
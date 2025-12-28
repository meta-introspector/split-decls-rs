macro_rules! test_flat_map {
    () => {
        # [test] fn test_flat_map () { let input : & [u8] = & [3 , 100 , 101 , 102 , 103 , 104] [..] ; assert_parse ! (flat_map (u8 , take) . parse (input) , Ok ((& [103 , 104] [..] , & [100 , 101 , 102] [..]))) ; }
    };
}

test_flat_map!();
macro_rules! test_find {
    () => {
        # [test] fn test_find () { let mut v = vec ! [0 , 1 , 2 , 3 , 1 , 2 , 1] ; assert_eq ! (v . rfind_remove (& 1) , Some ((6 , 1))) ; assert_eq ! (v . find_remove (& 2) , Some ((2 , 2))) ; assert_eq ! (v . find_remove (& 7) , None) ; assert_eq ! (& v , & [0 , 1 , 3 , 1 , 2]) ; }
    };
}

test_find!();
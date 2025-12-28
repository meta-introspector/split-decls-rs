macro_rules! test_common_suffix {
    () => {
        # [test] fn test_common_suffix () { let text1 = range ! ("abc") ; let text2 = range ! ("xyz") ; assert_eq ! (0 , common_suffix (text1 , text2) , "Null case") ; let text1 = range ! ("abcdef1234") ; let text2 = range ! ("xyz1234") ; assert_eq ! (4 , common_suffix (text1 , text2) , "Non-null case") ; let text1 = range ! ("1234") ; let text2 = range ! ("xyz1234") ; assert_eq ! (4 , common_suffix (text1 , text2) , "Whole case") ; }
    };
}

test_common_suffix!();
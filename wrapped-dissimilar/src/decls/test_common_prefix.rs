macro_rules! test_common_prefix {
    () => {
        # [test] fn test_common_prefix () { let text1 = range ! ("abc") ; let text2 = range ! ("xyz") ; assert_eq ! (0 , common_prefix (text1 , text2) , "Null case") ; let text1 = range ! ("1234abcdef") ; let text2 = range ! ("1234xyz") ; assert_eq ! (4 , common_prefix (text1 , text2) , "Non-null case") ; let text1 = range ! ("1234") ; let text2 = range ! ("1234xyz") ; assert_eq ! (4 , common_prefix (text1 , text2) , "Whole case") ; }
    };
}

test_common_prefix!();
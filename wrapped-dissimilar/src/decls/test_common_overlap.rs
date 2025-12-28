macro_rules! deps {
    () => {
        Range!();
    };
}

macro_rules! test_common_overlap {
    () => {
        deps!();
        # [test] fn test_common_overlap () { let text1 = Range :: empty () ; let text2 = range ! ("abcd") ; assert_eq ! (0 , common_overlap (text1 , text2) , "Null case") ; let text1 = range ! ("abc") ; let text2 = range ! ("abcd") ; assert_eq ! (3 , common_overlap (text1 , text2) , "Whole case") ; let text1 = range ! ("123456") ; let text2 = range ! ("abcd") ; assert_eq ! (0 , common_overlap (text1 , text2) , "No overlap") ; let text1 = range ! ("123456xxx") ; let text2 = range ! ("xxxabcd") ; assert_eq ! (3 , common_overlap (text1 , text2) , "Overlap") ; let text1 = range ! ("fi") ; let text2 = range ! ("\u{fb01}i") ; assert_eq ! (0 , common_overlap (text1 , text2) , "Unicode") ; }
    };
}

test_common_overlap!()
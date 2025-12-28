macro_rules! deps {
    () => {
        Solution!();
    };
}

macro_rules! test_bisect {
    () => {
        deps!();
        # [test] fn test_bisect () { let text1 = range ! ("cat") ; let text2 = range ! ("map") ; let solution = Solution { text1 , text2 , diffs : bisect (text1 , text2) , } ; assert_diffs ! ([Delete ("c") , Insert ("m") , Equal ("a") , Delete ("t") , Insert ("p") ,] , solution , "Normal" ,) ; }
    };
}

test_bisect!()
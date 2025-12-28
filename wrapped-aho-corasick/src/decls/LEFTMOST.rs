macro_rules! deps {
    () => {
        SearchTest!();
    };
}

macro_rules! LEFTMOST {
    () => {
        deps!();
        # [doc = " Tests for non-overlapping leftmost match semantics. These should pass for"] # [doc = " both leftmost-first and leftmost-longest match kinds. Stated differently,"] # [doc = " among ambiguous matches, the longest match and the match that appeared"] # [doc = " first when constructing the automaton should always be the same."] const LEFTMOST : & 'static [SearchTest] = & [t ! (leftmost000 , & ["ab" , "ab"] , "abcd" , & [(0 , 0 , 2)]) , t ! (leftmost010 , & ["a" , ""] , "a" , & [(0 , 0 , 1)]) , t ! (leftmost011 , & ["a" , ""] , "ab" , & [(0 , 0 , 1) , (1 , 2 , 2)]) , t ! (leftmost020 , & ["" , ""] , "a" , & [(0 , 0 , 0) , (0 , 1 , 1)]) , t ! (leftmost030 , & ["a" , "ab"] , "aa" , & [(0 , 0 , 1) , (0 , 1 , 2)]) , t ! (leftmost031 , & ["ab" , "a"] , "aa" , & [(1 , 0 , 1) , (1 , 1 , 2)]) , t ! (leftmost032 , & ["ab" , "a"] , "xayabbbz" , & [(1 , 1 , 2) , (0 , 3 , 5)]) , t ! (leftmost300 , & ["abcd" , "bce" , "b"] , "abce" , & [(1 , 1 , 4)]) , t ! (leftmost310 , & ["abcd" , "ce" , "bc"] , "abce" , & [(2 , 1 , 3)]) , t ! (leftmost320 , & ["abcd" , "bce" , "ce" , "b"] , "abce" , & [(1 , 1 , 4)]) , t ! (leftmost330 , & ["abcd" , "bce" , "cz" , "bc"] , "abcz" , & [(3 , 1 , 3)]) , t ! (leftmost340 , & ["bce" , "cz" , "bc"] , "bcz" , & [(2 , 0 , 2)]) , t ! (leftmost350 , & ["abc" , "bd" , "ab"] , "abd" , & [(2 , 0 , 2)]) , t ! (leftmost360 , & ["abcdefghi" , "hz" , "abcdefgh"] , "abcdefghz" , & [(2 , 0 , 8) ,]) , t ! (leftmost370 , & ["abcdefghi" , "cde" , "hz" , "abcdefgh"] , "abcdefghz" , & [(3 , 0 , 8) ,]) , t ! (leftmost380 , & ["abcdefghi" , "hz" , "abcdefgh" , "a"] , "abcdefghz" , & [(2 , 0 , 8) ,]) , t ! (leftmost390 , & ["b" , "abcdefghi" , "hz" , "abcdefgh"] , "abcdefghz" , & [(3 , 0 , 8) ,]) , t ! (leftmost400 , & ["h" , "abcdefghi" , "hz" , "abcdefgh"] , "abcdefghz" , & [(3 , 0 , 8) ,]) , t ! (leftmost410 , & ["z" , "abcdefghi" , "hz" , "abcdefgh"] , "abcdefghz" , & [(3 , 0 , 8) , (0 , 8 , 9) ,]) ,] ;
    };
}

LEFTMOST!()
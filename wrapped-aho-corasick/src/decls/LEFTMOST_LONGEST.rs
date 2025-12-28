macro_rules! deps {
    () => {
        SearchTest!();
    };
}

macro_rules! LEFTMOST_LONGEST {
    () => {
        deps!();
        # [doc = " Tests for non-overlapping leftmost-longest match semantics. These tests"] # [doc = " should generally be specific to leftmost-longest, which means they should"] # [doc = " generally fail under leftmost-first semantics."] const LEFTMOST_LONGEST : & 'static [SearchTest] = & [t ! (leftlong000 , & ["ab" , "abcd"] , "abcd" , & [(1 , 0 , 4)]) , t ! (leftlong010 , & ["abcd" , "bcd" , "cd" , "b"] , "abcd" , & [(0 , 0 , 4) ,]) , t ! (leftlong020 , & ["" , "a"] , "a" , & [(1 , 0 , 1)]) , t ! (leftlong021 , & ["" , "a" , ""] , "a" , & [(1 , 0 , 1)]) , t ! (leftlong022 , & ["a" , "" , ""] , "a" , & [(0 , 0 , 1)]) , t ! (leftlong023 , & ["" , "" , "a"] , "a" , & [(2 , 0 , 1)]) , t ! (leftlong024 , & ["" , "a"] , "ab" , & [(1 , 0 , 1) , (0 , 2 , 2)]) , t ! (leftlong030 , & ["" , "a"] , "aa" , & [(1 , 0 , 1) , (1 , 1 , 2)]) , t ! (leftlong040 , & ["a" , "ab"] , "a" , & [(0 , 0 , 1)]) , t ! (leftlong050 , & ["a" , "ab"] , "ab" , & [(1 , 0 , 2)]) , t ! (leftlong060 , & ["ab" , "a"] , "a" , & [(1 , 0 , 1)]) , t ! (leftlong070 , & ["ab" , "a"] , "ab" , & [(0 , 0 , 2)]) , t ! (leftlong100 , & ["abcdefg" , "bcde" , "bcdef"] , "abcdef" , & [(2 , 1 , 6)]) , t ! (leftlong110 , & ["abcdefg" , "bcdef" , "bcde"] , "abcdef" , & [(1 , 1 , 6)]) , t ! (leftlong300 , & ["abcd" , "b" , "bce"] , "abce" , & [(2 , 1 , 4)]) , t ! (leftlong310 , & ["a" , "abcdefghi" , "hz" , "abcdefgh"] , "abcdefghz" , & [(3 , 0 , 8) ,]) , t ! (leftlong320 , & ["a" , "abab"] , "abab" , & [(1 , 0 , 4)]) , t ! (leftlong330 , & ["abcd" , "b" , "ce"] , "abce" , & [(1 , 1 , 2) , (2 , 2 , 4) ,]) , t ! (leftlong340 , & ["a" , "ab"] , "xayabbbz" , & [(0 , 1 , 2) , (1 , 3 , 5)]) ,] ;
    };
}

LEFTMOST_LONGEST!();
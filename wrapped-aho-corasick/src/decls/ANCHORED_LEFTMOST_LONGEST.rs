macro_rules! deps {
    () => {
        SearchTest!();
    };
}

macro_rules! ANCHORED_LEFTMOST_LONGEST {
    () => {
        deps!();
        # [doc = " Like LEFTMOST_LONGEST, but for anchored searches."] const ANCHORED_LEFTMOST_LONGEST : & 'static [SearchTest] = & [t ! (aleftlong000 , & ["ab" , "abcd"] , "abcd" , & [(1 , 0 , 4)]) , t ! (aleftlong010 , & ["abcd" , "bcd" , "cd" , "b"] , "abcd" , & [(0 , 0 , 4) ,]) , t ! (aleftlong020 , & ["" , "a"] , "a" , & [(1 , 0 , 1)]) , t ! (aleftlong021 , & ["" , "a" , ""] , "a" , & [(1 , 0 , 1)]) , t ! (aleftlong022 , & ["a" , "" , ""] , "a" , & [(0 , 0 , 1)]) , t ! (aleftlong023 , & ["" , "" , "a"] , "a" , & [(2 , 0 , 1)]) , t ! (aleftlong030 , & ["" , "a"] , "aa" , & [(1 , 0 , 1) , (1 , 1 , 2)]) , t ! (aleftlong040 , & ["a" , "ab"] , "a" , & [(0 , 0 , 1)]) , t ! (aleftlong050 , & ["a" , "ab"] , "ab" , & [(1 , 0 , 2)]) , t ! (aleftlong060 , & ["ab" , "a"] , "a" , & [(1 , 0 , 1)]) , t ! (aleftlong070 , & ["ab" , "a"] , "ab" , & [(0 , 0 , 2)]) , t ! (aleftlong100 , & ["abcdefg" , "bcde" , "bcdef"] , "abcdef" , & []) , t ! (aleftlong110 , & ["abcdefg" , "bcdef" , "bcde"] , "abcdef" , & []) , t ! (aleftlong300 , & ["abcd" , "b" , "bce"] , "abce" , & []) , t ! (aleftlong310 , & ["a" , "abcdefghi" , "hz" , "abcdefgh"] , "abcdefghz" , & [(3 , 0 , 8) ,]) , t ! (aleftlong320 , & ["a" , "abab"] , "abab" , & [(1 , 0 , 4)]) , t ! (aleftlong330 , & ["abcd" , "b" , "ce"] , "abce" , & []) , t ! (aleftlong340 , & ["a" , "ab"] , "xayabbbz" , & []) ,] ;
    };
}

ANCHORED_LEFTMOST_LONGEST!();
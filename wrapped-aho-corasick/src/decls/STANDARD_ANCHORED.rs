macro_rules! deps {
    () => {
        SearchTest!();
    };
}

macro_rules! STANDARD_ANCHORED {
    () => {
        deps!();
        # [doc = " Like STANDARD, but for anchored searches."] const STANDARD_ANCHORED : & 'static [SearchTest] = & [t ! (astandard000 , & ["ab" , "abcd"] , "abcd" , & [(0 , 0 , 2)]) , t ! (astandard010 , & ["abcd" , "ab"] , "abcd" , & [(1 , 0 , 2)]) , t ! (astandard020 , & ["abcd" , "ab" , "abc"] , "abcd" , & [(1 , 0 , 2)]) , t ! (astandard030 , & ["abcd" , "abc" , "ab"] , "abcd" , & [(2 , 0 , 2)]) , t ! (astandard040 , & ["a" , ""] , "a" , & [(1 , 0 , 0) , (1 , 1 , 1)]) , t ! (astandard050 , & ["abcd" , "bcd" , "cd" , "b"] , "abcd" , & [(0 , 0 , 4)]) , t ! (astandard410 , & ["" , "a"] , "a" , & [(0 , 0 , 0) , (0 , 1 , 1)]) , t ! (astandard420 , & ["" , "a"] , "aa" , & [(0 , 0 , 0) , (0 , 1 , 1) , (0 , 2 , 2)]) , t ! (astandard430 , & ["" , "a" , ""] , "a" , & [(0 , 0 , 0) , (0 , 1 , 1)]) , t ! (astandard440 , & ["a" , "" , ""] , "a" , & [(1 , 0 , 0) , (1 , 1 , 1)]) , t ! (astandard450 , & ["" , "" , "a"] , "a" , & [(0 , 0 , 0) , (0 , 1 , 1)]) ,] ;
    };
}

STANDARD_ANCHORED!();
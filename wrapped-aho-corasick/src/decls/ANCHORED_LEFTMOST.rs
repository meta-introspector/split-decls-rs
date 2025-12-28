macro_rules! deps {
    () => {
        SearchTest!();
    };
}

macro_rules! ANCHORED_LEFTMOST {
    () => {
        deps!();
        # [doc = " Like LEFTMOST, but for anchored searches."] const ANCHORED_LEFTMOST : & 'static [SearchTest] = & [t ! (aleftmost000 , & ["ab" , "ab"] , "abcd" , & [(0 , 0 , 2)]) , t ! (aleftmost010 , & ["a" , ""] , "a" , & [(0 , 0 , 1)]) , t ! (aleftmost020 , & ["" , ""] , "a" , & [(0 , 0 , 0) , (0 , 1 , 1)]) , t ! (aleftmost030 , & ["a" , "ab"] , "aa" , & [(0 , 0 , 1) , (0 , 1 , 2)]) , t ! (aleftmost031 , & ["ab" , "a"] , "aa" , & [(1 , 0 , 1) , (1 , 1 , 2)]) , t ! (aleftmost032 , & ["ab" , "a"] , "xayabbbz" , & []) , t ! (aleftmost300 , & ["abcd" , "bce" , "b"] , "abce" , & []) , t ! (aleftmost301 , & ["abcd" , "bcd" , "cd" , "b"] , "abcd" , & [(0 , 0 , 4)]) , t ! (aleftmost310 , & ["abcd" , "ce" , "bc"] , "abce" , & []) , t ! (aleftmost320 , & ["abcd" , "bce" , "ce" , "b"] , "abce" , & []) , t ! (aleftmost330 , & ["abcd" , "bce" , "cz" , "bc"] , "abcz" , & []) , t ! (aleftmost340 , & ["bce" , "cz" , "bc"] , "bcz" , & [(2 , 0 , 2)]) , t ! (aleftmost350 , & ["abc" , "bd" , "ab"] , "abd" , & [(2 , 0 , 2)]) , t ! (aleftmost360 , & ["abcdefghi" , "hz" , "abcdefgh"] , "abcdefghz" , & [(2 , 0 , 8) ,]) , t ! (aleftmost370 , & ["abcdefghi" , "cde" , "hz" , "abcdefgh"] , "abcdefghz" , & [(3 , 0 , 8) ,]) , t ! (aleftmost380 , & ["abcdefghi" , "hz" , "abcdefgh" , "a"] , "abcdefghz" , & [(2 , 0 , 8) ,]) , t ! (aleftmost390 , & ["b" , "abcdefghi" , "hz" , "abcdefgh"] , "abcdefghz" , & [(3 , 0 , 8) ,]) , t ! (aleftmost400 , & ["h" , "abcdefghi" , "hz" , "abcdefgh"] , "abcdefghz" , & [(3 , 0 , 8) ,]) , t ! (aleftmost410 , & ["z" , "abcdefghi" , "hz" , "abcdefgh"] , "abcdefghzyz" , & [(3 , 0 , 8) , (0 , 8 , 9)]) ,] ;
    };
}

ANCHORED_LEFTMOST!();
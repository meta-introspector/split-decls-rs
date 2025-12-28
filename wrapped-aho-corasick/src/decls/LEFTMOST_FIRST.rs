macro_rules! deps {
    () => {
        SearchTest!();
    };
}

macro_rules! LEFTMOST_FIRST {
    () => {
        deps!();
        # [doc = " Tests for non-overlapping leftmost-first match semantics. These tests"] # [doc = " should generally be specific to leftmost-first, which means they should"] # [doc = " generally fail under leftmost-longest semantics."] const LEFTMOST_FIRST : & 'static [SearchTest] = & [t ! (leftfirst000 , & ["ab" , "abcd"] , "abcd" , & [(0 , 0 , 2)]) , t ! (leftfirst010 , & ["" , "a"] , "a" , & [(0 , 0 , 0) , (0 , 1 , 1)]) , t ! (leftfirst011 , & ["" , "a" , ""] , "a" , & [(0 , 0 , 0) , (0 , 1 , 1) ,]) , t ! (leftfirst012 , & ["a" , "" , ""] , "a" , & [(0 , 0 , 1)]) , t ! (leftfirst013 , & ["" , "" , "a"] , "a" , & [(0 , 0 , 0) , (0 , 1 , 1)]) , t ! (leftfirst014 , & ["a" , ""] , "a" , & [(0 , 0 , 1)]) , t ! (leftfirst015 , & ["a" , ""] , "ab" , & [(0 , 0 , 1) , (1 , 2 , 2)]) , t ! (leftfirst020 , & ["abcd" , "ab"] , "abcd" , & [(0 , 0 , 4)]) , t ! (leftfirst030 , & ["ab" , "ab"] , "abcd" , & [(0 , 0 , 2)]) , t ! (leftfirst040 , & ["a" , "ab"] , "xayabbbz" , & [(0 , 1 , 2) , (0 , 3 , 4)]) , t ! (leftfirst100 , & ["abcdefg" , "bcde" , "bcdef"] , "abcdef" , & [(1 , 1 , 5)]) , t ! (leftfirst110 , & ["abcdefg" , "bcdef" , "bcde"] , "abcdef" , & [(1 , 1 , 6)]) , t ! (leftfirst300 , & ["abcd" , "b" , "bce"] , "abce" , & [(1 , 1 , 2)]) , t ! (leftfirst310 , & ["abcd" , "b" , "bce" , "ce"] , "abce" , & [(1 , 1 , 2) , (3 , 2 , 4) ,]) , t ! (leftfirst320 , & ["a" , "abcdefghi" , "hz" , "abcdefgh"] , "abcdefghz" , & [(0 , 0 , 1) , (2 , 7 , 9) ,]) , t ! (leftfirst330 , & ["a" , "abab"] , "abab" , & [(0 , 0 , 1) , (0 , 2 , 3)]) , t ! (leftfirst400 , & ["amwix" , "samwise" , "sam"] , "Zsamwix" , & [(2 , 1 , 4)]) ,] ;
    };
}

LEFTMOST_FIRST!();
// Generated macro for ANCHORED_LEFTMOST_FIRST (const)
macro_rules! Depcrate_testsANCHORED_LEFTMOST_FIRST {
() => {
// Module: crate::tests
// Provides: {"ANCHORED_LEFTMOST_FIRST"}
// Dependencies: {}
# [doc = " Like LEFTMOST_FIRST, but for anchored searches."] const ANCHORED_LEFTMOST_FIRST : & 'static [SearchTest] = & [t ! (aleftfirst000 , & ["ab" , "abcd"] , "abcd" , & [(0 , 0 , 2)]) , t ! (aleftfirst010 , & ["" , "a"] , "a" , & [(0 , 0 , 0) , (0 , 1 , 1)]) , t ! (aleftfirst011 , & ["" , "a" , ""] , "a" , & [(0 , 0 , 0) , (0 , 1 , 1)]) , t ! (aleftfirst012 , & ["a" , "" , ""] , "a" , & [(0 , 0 , 1)]) , t ! (aleftfirst013 , & ["" , "" , "a"] , "a" , & [(0 , 0 , 0) , (0 , 1 , 1)]) , t ! (aleftfirst020 , & ["abcd" , "ab"] , "abcd" , & [(0 , 0 , 4)]) , t ! (aleftfirst030 , & ["ab" , "ab"] , "abcd" , & [(0 , 0 , 2)]) , t ! (aleftfirst040 , & ["a" , "ab"] , "xayabbbz" , & []) , t ! (aleftfirst100 , & ["abcdefg" , "bcde" , "bcdef"] , "abcdef" , & []) , t ! (aleftfirst110 , & ["abcdefg" , "bcdef" , "bcde"] , "abcdef" , & []) , t ! (aleftfirst300 , & ["abcd" , "b" , "bce"] , "abce" , & []) , t ! (aleftfirst310 , & ["abcd" , "b" , "bce" , "ce"] , "abce" , & []) , t ! (aleftfirst320 , & ["a" , "abcdefghi" , "hz" , "abcdefgh"] , "abcdefghz" , & [(0 , 0 , 1)]) , t ! (aleftfirst330 , & ["a" , "abab"] , "abab" , & [(0 , 0 , 1)]) , t ! (aleftfirst400 , & ["wise" , "samwise" , "sam"] , "samwix" , & [(2 , 0 , 3)]) ,] ;
};
}

macro_rules! deps {
    () => {
        SearchTest!();
    };
}

macro_rules! ANCHORED_BASICS {
    () => {
        deps!();
        # [doc = " A collection of *anchored* tests for the Aho-Corasick algorithm that should"] # [doc = " always be true regardless of match semantics. That is, all combinations of"] # [doc = " leftmost-{shortest, first, longest} x {overlapping, non-overlapping} should"] # [doc = " produce the same answer."] const ANCHORED_BASICS : & 'static [SearchTest] = & [t ! (abasic000 , & [] , "" , & []) , t ! (abasic001 , & [] , "a" , & []) , t ! (abasic002 , & [] , "abc" , & []) , t ! (abasic010 , & [""] , "" , & [(0 , 0 , 0)]) , t ! (abasic020 , & [""] , "a" , & [(0 , 0 , 0) , (0 , 1 , 1)]) , t ! (abasic030 , & [""] , "abc" , & [(0 , 0 , 0) , (0 , 1 , 1) , (0 , 2 , 2) , (0 , 3 , 3)]) , t ! (abasic100 , & ["a"] , "a" , & [(0 , 0 , 1)]) , t ! (abasic110 , & ["a"] , "aa" , & [(0 , 0 , 1) , (0 , 1 , 2)]) , t ! (abasic120 , & ["a" , "b"] , "ab" , & [(0 , 0 , 1) , (1 , 1 , 2)]) , t ! (abasic130 , & ["a" , "b"] , "ba" , & [(1 , 0 , 1) , (0 , 1 , 2)]) , t ! (abasic140 , & ["foo" , "foofoo"] , "foo" , & [(0 , 0 , 3)]) , t ! (abasic150 , & ["foofoo" , "foo"] , "foo" , & [(1 , 0 , 3)]) , t ! (abasic200 , & ["foo"] , "foofoo foo" , & [(0 , 0 , 3) , (0 , 3 , 6)]) ,] ;
    };
}

ANCHORED_BASICS!()
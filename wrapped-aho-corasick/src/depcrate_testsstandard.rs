// Generated macro for STANDARD (const)
macro_rules! Depcrate_testsSTANDARD {
() => {
// Module: crate::tests
// Provides: {"STANDARD"}
// Dependencies: {}
# [doc = " Tests for non-overlapping standard match semantics."] # [doc = ""] # [doc = " These tests generally shouldn't pass for leftmost-{first,longest}, although"] # [doc = " some do in order to write clearer tests. For example, standard000 will"] # [doc = " pass with leftmost-first semantics, but standard010 will not. We write"] # [doc = " both to emphasize how the match semantics work."] const STANDARD : & 'static [SearchTest] = & [t ! (standard000 , & ["ab" , "abcd"] , "abcd" , & [(0 , 0 , 2)]) , t ! (standard010 , & ["abcd" , "ab"] , "abcd" , & [(1 , 0 , 2)]) , t ! (standard020 , & ["abcd" , "ab" , "abc"] , "abcd" , & [(1 , 0 , 2)]) , t ! (standard030 , & ["abcd" , "abc" , "ab"] , "abcd" , & [(2 , 0 , 2)]) , t ! (standard040 , & ["a" , ""] , "a" , & [(1 , 0 , 0) , (1 , 1 , 1)]) , t ! (standard400 , & ["abcd" , "bcd" , "cd" , "b"] , "abcd" , & [(3 , 1 , 2) , (2 , 2 , 4) ,]) , t ! (standard410 , & ["" , "a"] , "a" , & [(0 , 0 , 0) , (0 , 1 , 1) ,]) , t ! (standard420 , & ["" , "a"] , "aa" , & [(0 , 0 , 0) , (0 , 1 , 1) , (0 , 2 , 2) ,]) , t ! (standard430 , & ["" , "a" , ""] , "a" , & [(0 , 0 , 0) , (0 , 1 , 1) ,]) , t ! (standard440 , & ["a" , "" , ""] , "a" , & [(1 , 0 , 0) , (1 , 1 , 1) ,]) , t ! (standard450 , & ["" , "" , "a"] , "a" , & [(0 , 0 , 0) , (0 , 1 , 1) ,]) ,] ;
};
}

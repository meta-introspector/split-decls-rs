// Generated macro for ASCII_CASE_INSENSITIVE (const)
macro_rules! Depcrate_testsASCII_CASE_INSENSITIVE {
() => {
// Module: crate::tests
// Provides: {"ASCII_CASE_INSENSITIVE"}
// Dependencies: {}
# [doc = " Tests for ASCII case insensitivity."] # [doc = ""] # [doc = " These tests should all have the same behavior regardless of match semantics"] # [doc = " or whether the search is overlapping."] const ASCII_CASE_INSENSITIVE : & 'static [SearchTest] = & [t ! (acasei000 , & ["a"] , "A" , & [(0 , 0 , 1)]) , t ! (acasei010 , & ["Samwise"] , "SAMWISE" , & [(0 , 0 , 7)]) , t ! (acasei011 , & ["Samwise"] , "SAMWISE.abcd" , & [(0 , 0 , 7)]) , t ! (acasei020 , & ["fOoBaR"] , "quux foobar baz" , & [(0 , 5 , 11)]) ,] ;
};
}

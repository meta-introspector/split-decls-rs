// Generated macro for ASCII_CASE_INSENSITIVE_NON_OVERLAPPING (const)
macro_rules! Depcrate_testsASCII_CASE_INSENSITIVE_NON_OVERLAPPING {
() => {
// Module: crate::tests
// Provides: {"ASCII_CASE_INSENSITIVE_NON_OVERLAPPING"}
// Dependencies: {}
# [doc = " Like ASCII_CASE_INSENSITIVE, but specifically for non-overlapping tests."] const ASCII_CASE_INSENSITIVE_NON_OVERLAPPING : & 'static [SearchTest] = & [t ! (acasei000 , & ["foo" , "FOO"] , "fOo" , & [(0 , 0 , 3)]) , t ! (acasei000 , & ["FOO" , "foo"] , "fOo" , & [(0 , 0 , 3)]) , t ! (acasei010 , & ["abc" , "def"] , "abcdef" , & [(0 , 0 , 3) , (1 , 3 , 6)]) ,] ;
};
}

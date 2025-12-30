// Generated macro for arbitrary_collection (function)
macro_rules! Depcrate_testsarbitrary_collection {
() => {
// Module: crate::tests
// Provides: {"arbitrary_collection"}
// Dependencies: {}
# [test] fn arbitrary_collection () { let x = [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 8 , 12 ,] ; assert_eq ! (checked_arbitrary ::<& [u8] > (& mut Unstructured :: new (& x)) . unwrap () , & [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 1 , 2 , 3]) ; assert_eq ! (checked_arbitrary ::< Vec < u8 >> (& mut Unstructured :: new (& x)) . unwrap () , & [2 , 4 , 6 , 8 , 1]) ; assert_eq ! (&* checked_arbitrary ::< Box < [u8] >> (& mut Unstructured :: new (& x)) . unwrap () , & [2 , 4 , 6 , 8 , 1]) ; assert_eq ! (&* checked_arbitrary ::< Arc < [u8] >> (& mut Unstructured :: new (& x)) . unwrap () , & [2 , 4 , 6 , 8 , 1]) ; assert_eq ! (&* checked_arbitrary ::< Rc < [u8] >> (& mut Unstructured :: new (& x)) . unwrap () , & [2 , 4 , 6 , 8 , 1]) ; assert_eq ! (checked_arbitrary ::< Vec < u32 >> (& mut Unstructured :: new (& x)) . unwrap () , & [84148994]) ; assert_eq ! (checked_arbitrary ::< String > (& mut Unstructured :: new (& x)) . unwrap () , "\x01\x02\x03\x04\x05\x06\x07\x08\x09\x01\x02\x03") ; }
};
}

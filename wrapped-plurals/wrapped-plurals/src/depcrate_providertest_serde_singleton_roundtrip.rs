// Generated macro for test_serde_singleton_roundtrip (function)
macro_rules! Depcrate_providertest_serde_singleton_roundtrip {
() => {
// Module: crate::provider
// Provides: {"test_serde_singleton_roundtrip"}
// Dependencies: {}
# [test] fn test_serde_singleton_roundtrip () { let plural_elements = PluralElements :: new ((FourBitMetadata :: zero () , "abc")) ; let ule = zerovec :: ule :: encode_varule_to_box (& plural_elements) ; let postcard_bytes = postcard :: to_allocvec (& ule) . unwrap () ; assert_eq ! (postcard_bytes , & [4 , 0x00 , b'a' , b'b' , b'c' ,]) ; let postcard_ule : Box < PluralElementsPackedULE < str > > = postcard :: from_bytes (& postcard_bytes) . unwrap () ; assert_eq ! (ule , postcard_ule) ; let postcard_borrowed : & PluralElementsPackedULE < str > = postcard :: from_bytes (& postcard_bytes) . unwrap () ; assert_eq ! (&* ule , postcard_borrowed) ; let postcard_cow : PluralElementsPackedCow < str > = postcard :: from_bytes (& postcard_bytes) . unwrap () ; assert_eq ! (&* ule , &* postcard_cow . elements) ; assert ! (matches ! (postcard_cow . elements , Cow :: Borrowed (_))) ; let json_str = serde_json :: to_string (& ule) . unwrap () ; let json_ule : Box < PluralElementsPackedULE < str > > = serde_json :: from_str (& json_str) . unwrap () ; assert_eq ! (ule , json_ule) ; }
};
}

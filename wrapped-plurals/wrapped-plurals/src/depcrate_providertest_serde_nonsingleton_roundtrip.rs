// Generated macro for test_serde_nonsingleton_roundtrip (function)
macro_rules! Depcrate_providertest_serde_nonsingleton_roundtrip {
() => {
// Module: crate::provider
// Provides: {"test_serde_nonsingleton_roundtrip"}
// Dependencies: {}
# [test] fn test_serde_nonsingleton_roundtrip () { let plural_elements = PluralElements :: new ((FourBitMetadata :: zero () , "abc")) . with_one_value (Some ((FourBitMetadata :: zero () , "defg"))) ; let ule = zerovec :: ule :: encode_varule_to_box (& plural_elements) ; let postcard_bytes = postcard :: to_allocvec (& ule) . unwrap () ; assert_eq ! (postcard_bytes , & [12 , 0x80 , 3 , b'a' , b'b' , b'c' , 1 , 0 , 0x10 , b'd' , b'e' , b'f' , b'g']) ; let postcard_ule : Box < PluralElementsPackedULE < str > > = postcard :: from_bytes (& postcard_bytes) . unwrap () ; assert_eq ! (ule , postcard_ule) ; let postcard_borrowed : & PluralElementsPackedULE < str > = postcard :: from_bytes (& postcard_bytes) . unwrap () ; assert_eq ! (&* ule , postcard_borrowed) ; let json_str = serde_json :: to_string (& ule) . unwrap () ; let json_ule : Box < PluralElementsPackedULE < str > > = serde_json :: from_str (& json_str) . unwrap () ; assert_eq ! (ule , json_ule) ; }
};
}

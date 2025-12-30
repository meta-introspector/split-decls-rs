// Generated macro for hash_serialization_test (macro)
macro_rules! Depcrate_devhash_serialization_test {
() => {
// Module: crate::dev
// Provides: {"hash_serialization_test"}
// Dependencies: {}
# [doc = " Define hash function serialization test"] # [macro_export] macro_rules ! hash_serialization_test { ($ name : ident , $ hasher : ty $ (,) ?) => { # [test] fn $ name () { use digest :: { Digest , crypto_common :: { BlockSizeUser , hazmat :: SerializableState } , typenum :: Unsigned , } ; let mut h = <$ hasher >:: new () ; h . update (& [0x13 ; <$ hasher as BlockSizeUser >:: BlockSize :: USIZE + 1]) ; let serialized_state = h . serialize () ; let expected = include_bytes ! (concat ! ("data/" , stringify ! ($ name) , ".bin")) ; assert_eq ! (serialized_state . as_slice () , expected) ; let mut h = <$ hasher >:: deserialize (& serialized_state) . unwrap () ; h . update (& [0x13 ; <$ hasher as BlockSizeUser >:: BlockSize :: USIZE + 1]) ; let output1 = h . finalize () ; let mut h = <$ hasher >:: new () ; h . update (& [0x13 ; 2 * (<$ hasher as BlockSizeUser >:: BlockSize :: USIZE + 1)]) ; let output2 = h . finalize () ; assert_eq ! (output1 , output2) ; } } ; }
};
}

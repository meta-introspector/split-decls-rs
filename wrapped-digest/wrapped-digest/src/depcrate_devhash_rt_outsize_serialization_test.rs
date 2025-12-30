// Generated macro for hash_rt_outsize_serialization_test (macro)
macro_rules! Depcrate_devhash_rt_outsize_serialization_test {
() => {
// Module: crate::dev
// Provides: {"hash_rt_outsize_serialization_test"}
// Dependencies: {}
# [doc = " Define hash function serialization test"] # [macro_export] macro_rules ! hash_rt_outsize_serialization_test { ($ name : ident , $ hasher : ty , $ expected_serialized_state : expr) => { # [test] fn $ name () { use digest :: { Digest , Update , VariableOutput , crypto_common :: { BlockSizeUser , hazmat :: SerializableState } , typenum :: Unsigned , } ; const HASH_OUTPUT_SIZE : usize = <$ hasher >:: MAX_OUTPUT_SIZE - 1 ; let mut h = <$ hasher >:: new (HASH_OUTPUT_SIZE) . unwrap () ; h . update (& [0x13 ; <$ hasher as BlockSizeUser >:: BlockSize :: USIZE + 1]) ; let serialized_state = h . serialize () ; assert_eq ! (serialized_state . as_slice () , $ expected_serialized_state) ; let mut h = <$ hasher >:: deserialize (& serialized_state) . unwrap () ; h . update (& [0x13 ; <$ hasher as BlockSizeUser >:: BlockSize :: USIZE + 1]) ; let mut output1 = [0 ; HASH_OUTPUT_SIZE] ; h . finalize_variable (& mut output1) . unwrap () ; let mut h = <$ hasher >:: new (HASH_OUTPUT_SIZE) . unwrap () ; h . update (& [0x13 ; 2 * (<$ hasher as BlockSizeUser >:: BlockSize :: USIZE + 1)]) ; let mut output2 = [0 ; HASH_OUTPUT_SIZE] ; h . finalize_variable (& mut output2) . unwrap () ; assert_eq ! (output1 , output2) ; } } ; }
};
}

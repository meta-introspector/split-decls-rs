// Generated macro for test_same (function)
macro_rules! Depcratetest_same {
() => {
// Module: crate
// Provides: {"test_same"}
// Dependencies: {}
pub fn test_same < T > (t : T) where T : bincode_2 :: Encode + bincode_2 :: Decode < () > + serde :: Serialize + serde :: de :: DeserializeOwned + core :: fmt :: Debug + PartialEq , { test_same_with_config (& t , bincode_1 :: options () . with_fixint_encoding () , bincode_2 :: config :: legacy () ,) ; test_same_with_config (& t , bincode_1 :: options () . with_big_endian () . with_varint_encoding () , bincode_2 :: config :: legacy () . with_big_endian () . with_variable_int_encoding () ,) ; test_same_with_config (& t , bincode_1 :: options () . with_little_endian () . with_varint_encoding () , bincode_2 :: config :: legacy () . with_little_endian () . with_variable_int_encoding () ,) ; test_same_with_config (& t , bincode_1 :: options () . with_big_endian () . with_fixint_encoding () , bincode_2 :: config :: legacy () . with_big_endian () . with_fixed_int_encoding () ,) ; test_same_with_config (& t , bincode_1 :: options () . with_little_endian () . with_fixint_encoding () , bincode_2 :: config :: legacy () . with_little_endian () . with_fixed_int_encoding () ,) ; }
};
}

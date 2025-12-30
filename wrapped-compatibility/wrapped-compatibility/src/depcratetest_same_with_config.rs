// Generated macro for test_same_with_config (function)
macro_rules! Depcratetest_same_with_config {
() => {
// Module: crate
// Provides: {"test_same_with_config"}
// Dependencies: {}
pub fn test_same_with_config < T , C , O > (t : & T , bincode_1_options : O , bincode_2_config : C) where T : bincode_2 :: Encode + bincode_2 :: Decode < () > + serde :: Serialize + serde :: de :: DeserializeOwned + core :: fmt :: Debug + PartialEq , C : bincode_2 :: config :: Config , O : bincode_1 :: Options + Copy , { let encoded = bincode_1_options . serialize (t) . unwrap () ; println ! ("Encoded {t:?} as {encoded:?}") ; let bincode_2_output = bincode_2 :: encode_to_vec (t , bincode_2_config) . unwrap () ; assert_eq ! (encoded , bincode_2_output , "{t:?} serializes differently\nbincode 2 config {:?}" , core :: any :: type_name ::< C > () ,) ; let bincode_2_serde_output = bincode_2 :: serde :: encode_to_vec (t , bincode_2_config) . unwrap () ; assert_eq ! (encoded , bincode_2_serde_output , "{t:?} serializes differently") ; let decoded : T = bincode_1_options . deserialize (& encoded) . unwrap () ; assert_eq ! (& decoded , t) ; let decoded : T = bincode_2 :: decode_from_slice (& encoded , bincode_2_config) . unwrap () . 0 ; assert_eq ! (& decoded , t) ; let decoded : T = bincode_2 :: serde :: decode_from_slice (& encoded , bincode_2_config) . unwrap () . 0 ; assert_eq ! (& decoded , t) ; }
};
}

// Generated macro for test_counter_kbkdfvs (function)
macro_rules! Depcrate_teststest_counter_kbkdfvs {
() => {
// Module: crate::tests
// Provides: {"test_counter_kbkdfvs"}
// Dependencies: {}
# [test] fn test_counter_kbkdfvs () { type HmacSha256 = hmac :: Hmac < sha2 :: Sha256 > ; struct MockOutput ; impl KeySizeUser for MockOutput { type KeySize = U32 ; } let counter = Counter :: < HmacSha256 , MockOutput > :: default () ; assert_eq ! (counter . derive (Params :: builder (& hex ! ("43eef6d824fd820405626ab9b6d79f1fd04e126ab8e17729e3afc7cb5af794f8")) . use_l (false) . use_separator (false) . with_label (& hex ! ("5e269b5a7bdedcc3e875e2725693a257fc60011af7dcd68a3358507fe29b0659" "ca66951daa05a15032033650bc58a27840f8fbe9f4088b9030738f68")) . build ()) , Ok (Array ::< u8 , U32 >:: from (hex ! ("f0a339ecbcae6add1afb27da3ba40a1320c6427a58afb9dc366b219b7eb29ecf"))) ,) ; }
};
}

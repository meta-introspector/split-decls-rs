// Generated macro for check_from_str_bytes_reader (function)
macro_rules! Depcrate_de_testscheck_from_str_bytes_reader {
() => {
// Module: crate::de::tests
// Provides: {"check_from_str_bytes_reader"}
// Dependencies: {}
fn check_from_str_bytes_reader < T : serde :: de :: DeserializeOwned + PartialEq + core :: fmt :: Debug > (ron : & str , check : SpannedResult < T > ,) { let res_str = super :: from_str :: < T > (ron) ; assert_eq ! (res_str , check) ; let res_bytes = super :: from_bytes :: < T > (ron . as_bytes ()) ; assert_eq ! (res_bytes , check) ; # [cfg (feature = "std")] { let res_reader = super :: from_reader :: < & [u8] , T > (ron . as_bytes ()) ; assert_eq ! (res_reader , check) ; } }
};
}

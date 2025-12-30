// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn big_mod_exp_test () { # [derive (serde_derive :: Deserialize)] # [serde (rename_all = "PascalCase")] struct TestCase { base : String , exponent : String , modulus : String , expected : String , } let test_data = include_str ! ("../tests/data/big_mod_exp_cases.json") ; let test_cases : Vec < TestCase > = serde_json :: from_str (test_data) . unwrap () ; test_cases . iter () . for_each (| test | { let base = array_bytes :: hex2bytes_unchecked (& test . base) ; let exponent = array_bytes :: hex2bytes_unchecked (& test . exponent) ; let modulus = array_bytes :: hex2bytes_unchecked (& test . modulus) ; let expected = array_bytes :: hex2bytes_unchecked (& test . expected) ; let result = big_mod_exp (base . as_slice () , exponent . as_slice () , modulus . as_slice ()) ; assert_eq ! (result , expected) ; }) ; } }
};
}

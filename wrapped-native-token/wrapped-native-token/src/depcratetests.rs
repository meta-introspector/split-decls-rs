// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_sol_str_to_lamports () { assert_eq ! (0 , sol_str_to_lamports ("0.0") . unwrap ()) ; assert_eq ! (1 , sol_str_to_lamports ("0.000000001") . unwrap ()) ; assert_eq ! (10 , sol_str_to_lamports ("0.00000001") . unwrap ()) ; assert_eq ! (100 , sol_str_to_lamports ("0.0000001") . unwrap ()) ; assert_eq ! (1000 , sol_str_to_lamports ("0.000001") . unwrap ()) ; assert_eq ! (10000 , sol_str_to_lamports ("0.00001") . unwrap ()) ; assert_eq ! (100000 , sol_str_to_lamports ("0.0001") . unwrap ()) ; assert_eq ! (1000000 , sol_str_to_lamports ("0.001") . unwrap ()) ; assert_eq ! (10000000 , sol_str_to_lamports ("0.01") . unwrap ()) ; assert_eq ! (100000000 , sol_str_to_lamports ("0.1") . unwrap ()) ; assert_eq ! (1000000000 , sol_str_to_lamports ("1") . unwrap ()) ; assert_eq ! (4_100_000_000 , sol_str_to_lamports ("4.1") . unwrap ()) ; assert_eq ! (8_200_000_000 , sol_str_to_lamports ("8.2") . unwrap ()) ; assert_eq ! (8_502_282_880 , sol_str_to_lamports ("8.50228288") . unwrap ()) ; assert_eq ! (u64 :: MAX , sol_str_to_lamports ("18446744073.709551615") . unwrap ()) ; assert_eq ! (None , sol_str_to_lamports ("18446744073.709551616")) ; assert_eq ! (None , sol_str_to_lamports ("-0.000000001")) ; assert_eq ! (None , sol_str_to_lamports ("-9223372036.854775808")) ; } }
};
}

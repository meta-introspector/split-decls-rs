// Generated macro for tests (module)
macro_rules! Depcrate_utiltests {
() => {
// Module: crate::util
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn custom_decimal_format () { let fmt = | n : u64 | { let bytes = DecimalFormatter :: new (n) . as_bytes () . to_vec () ; String :: from_utf8 (bytes) . unwrap () } ; let std = | n : u64 | n . to_string () ; let ints = [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 20 , 100 , 123 , u64 :: MAX] ; for n in ints { assert_eq ! (std (n) , fmt (n)) ; } } }
};
}

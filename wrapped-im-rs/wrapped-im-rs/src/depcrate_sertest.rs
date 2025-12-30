// Generated macro for test (module)
macro_rules! Depcrate_sertest {
() => {
// Module: crate::ser
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: proptest :: { hash_map , hash_set , ord_map , ord_set , vector } ; use :: proptest :: num :: i32 ; use :: proptest :: proptest ; use serde_json :: { from_str , to_string } ; proptest ! { # [test] fn ser_ordset (ref v in ord_set (i32 :: ANY , 0 .. 100)) { assert_eq ! (v , & from_str ::< OrdSet < i32 >> (& to_string (& v) . unwrap ()) . unwrap ()) ; } # [test] fn ser_ordmap (ref v in ord_map (i32 :: ANY , i32 :: ANY , 0 .. 100)) { assert_eq ! (v , & from_str ::< OrdMap < i32 , i32 >> (& to_string (& v) . unwrap ()) . unwrap ()) ; } # [test] fn ser_hashmap (ref v in hash_map (i32 :: ANY , i32 :: ANY , 0 .. 100)) { assert_eq ! (v , & from_str ::< HashMap < i32 , i32 >> (& to_string (& v) . unwrap ()) . unwrap ()) ; } # [test] fn ser_hashset (ref v in hash_set (i32 :: ANY , 0 .. 100)) { assert_eq ! (v , & from_str ::< HashSet < i32 >> (& to_string (& v) . unwrap ()) . unwrap ()) ; } # [test] fn ser_vector (ref v in vector (i32 :: ANY , 0 .. 100)) { assert_eq ! (v , & from_str ::< Vector < i32 >> (& to_string (& v) . unwrap ()) . unwrap ()) ; } } }
};
}

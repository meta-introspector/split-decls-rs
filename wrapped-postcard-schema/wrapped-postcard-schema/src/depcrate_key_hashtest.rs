// Generated macro for test (module)
macro_rules! Depcrate_key_hashtest {
() => {
// Module: crate::key::hash
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: fnv1a64 :: hash_ty_path ; # [test] fn type_punning_good () { let hash_1 = hash_ty_path :: < Vec < u8 > > ("test_path") ; let hash_2 = hash_ty_path :: < & [u8] > ("test_path") ; let hash_3 = hash_ty_path :: < Vec < u16 > > ("test_path") ; let hash_4 = hash_ty_path :: < & [u16] > ("test_path") ; let hash_5 = hash_ty_path :: < Vec < u8 > > ("test_patt") ; let hash_6 = hash_ty_path :: < & [u8] > ("test_patt") ; assert_eq ! (hash_1 , hash_2) ; assert_eq ! (hash_3 , hash_4) ; assert_ne ! (hash_1 , hash_3) ; assert_ne ! (hash_2 , hash_4) ; assert_ne ! (hash_1 , hash_5) ; assert_ne ! (hash_2 , hash_6) ; } }
};
}

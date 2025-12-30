// Generated macro for check_equivalence (function)
macro_rules! Depcrate_testingcheck_equivalence {
() => {
// Module: crate::testing
// Provides: {"check_equivalence"}
// Dependencies: {}
# [expect (clippy :: expect_used)] fn check_equivalence < 'a , K , V , S0 , S1 > (mut a : S0 , mut b : S1) where K : Ord + Debug + PartialEq + 'a , V : Debug + PartialEq + 'a , S0 : StoreMut < K , V > + StoreIterable < 'a , K , V > , S1 : StoreMut < K , V > + StoreIterable < 'a , K , V > , { let len = a . lm_len () ; assert_eq ! (len , b . lm_len ()) ; if len == 0 { assert ! (a . lm_is_empty ()) ; assert ! (b . lm_is_empty ()) ; } for i in 0 .. len { let a_kv = a . lm_get (i) ; let b_kv = b . lm_get (i) ; assert ! (a_kv . is_some ()) ; assert_eq ! (a_kv , b_kv) ; let a_kv_mut = a . lm_get_mut (i) ; let b_kv_mut = b . lm_get_mut (i) ; assert ! (a_kv_mut . is_some ()) ; assert_eq ! (a_kv_mut , b_kv_mut) ; } for j in 0 .. len { let needle = a . lm_get (j) . expect ("j is in range") . 0 ; let a_binary = a . lm_binary_search_by (| k | k . cmp (needle)) ; let b_binary = a . lm_binary_search_by (| k | k . cmp (needle)) ; assert_eq ! (Ok (j) , a_binary) ; assert_eq ! (Ok (j) , b_binary) ; } assert ! (a . lm_get (len) . is_none ()) ; assert ! (b . lm_get (len) . is_none ()) ; assert_eq ! (a . lm_last () , b . lm_last ()) ; }
};
}

// Generated macro for check_into_iter_equivalence (function)
macro_rules! Depcrate_testingcheck_into_iter_equivalence {
() => {
// Module: crate::testing
// Provides: {"check_into_iter_equivalence"}
// Dependencies: {}
fn check_into_iter_equivalence < K , V , S0 , S1 > (a : S0 , b : S1) where K : Ord + Debug + PartialEq , V : Debug + PartialEq , S0 : StoreIntoIterator < K , V > , S1 : StoreIntoIterator < K , V > , { let a_vec = a . lm_into_iter () . collect :: < Vec < _ > > () ; let b_vec = b . lm_into_iter () . collect :: < Vec < _ > > () ; assert_eq ! (a_vec , b_vec) ; }
};
}

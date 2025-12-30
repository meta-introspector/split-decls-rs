// Generated macro for check_btree_map (function)
macro_rules! Depcrate_iter_testcheck_btree_map {
() => {
// Module: crate::iter::test
// Provides: {"check_btree_map"}
// Dependencies: {}
# [test] fn check_btree_map () { use std :: collections :: BTreeMap ; let mut a : BTreeMap < i32 , i32 > = (0 .. 10) . map (| i | (i , - i)) . collect () ; assert_eq ! (45 , a . par_iter () . map (| (& k , _) | k) . sum ::< i32 > ()) ; assert_eq ! (- 45 , a . par_iter () . map (| (_ , & v) | v) . sum ::< i32 > ()) ; a . par_iter_mut () . for_each (| (k , v) | * v += * k) ; assert_eq ! (0 , a . into_par_iter () . map (| (_ , v) | v) . sum ::< i32 > ()) ; }
};
}

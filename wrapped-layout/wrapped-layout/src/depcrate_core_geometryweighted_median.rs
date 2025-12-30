// Generated macro for weighted_median (function)
macro_rules! Depcrate_core_geometryweighted_median {
() => {
// Module: crate::core::geometry
// Provides: {"weighted_median"}
// Dependencies: {}
# [doc = " Return the weighted median for \\p vec."] # [doc = " This is the method that's described in"] # [doc = " \"DAG - A Program that Draws Directed Graphs\""] # [doc = " Gansner, North, Vo 1989. Pg 10."] pub fn weighted_median (vec : & [f64]) -> f64 { assert ! (! vec . is_empty () , "array can't be empty") ; let mut vec = vec . to_vec () ; vec . sort_by (| a , b | a . partial_cmp (b) . unwrap ()) ; if vec . len () == 1 { return vec [0] ; } if vec . len () == 2 { return (vec [0] + vec [1]) / 2. ; } let mid = vec . len () / 2 ; if vec . len () % 2 == 1 { return vec [mid] ; } (vec [mid] + vec [mid - 1]) / 2. }
};
}

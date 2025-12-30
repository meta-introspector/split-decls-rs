// Generated macro for tests (module)
macro_rules! Depcrate_algo_dominatorstests {
() => {
// Module: crate::algo::dominators
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_iter_dominators () { let doms : Dominators < u32 > = Dominators { root : 0 , dominators : [(2 , 1) , (1 , 0) , (0 , 0)] . iter () . cloned () . collect () , } ; let all_doms : Vec < _ > = doms . dominators (2) . unwrap () . collect () ; assert_eq ! (vec ! [2 , 1 , 0] , all_doms) ; assert_eq ! (None ::< () >, doms . dominators (99) . map (| _ | unreachable ! ())) ; let strict_doms : Vec < _ > = doms . strict_dominators (2) . unwrap () . collect () ; assert_eq ! (vec ! [1 , 0] , strict_doms) ; assert_eq ! (None ::< () >, doms . strict_dominators (99) . map (| _ | unreachable ! ())) ; let dom_by : Vec < _ > = doms . immediately_dominated_by (1) . collect () ; assert_eq ! (vec ! [2] , dom_by) ; assert_eq ! (None , doms . immediately_dominated_by (99) . next ()) ; assert_eq ! (1 , doms . immediately_dominated_by (0) . count ()) ; } }
};
}

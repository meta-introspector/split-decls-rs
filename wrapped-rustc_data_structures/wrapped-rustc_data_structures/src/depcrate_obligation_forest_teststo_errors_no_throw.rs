// Generated macro for to_errors_no_throw (function)
macro_rules! Depcrate_obligation_forest_teststo_errors_no_throw {
() => {
// Module: crate::obligation_forest::tests
// Provides: {"to_errors_no_throw"}
// Dependencies: {}
# [test] fn to_errors_no_throw () { let mut forest = ObligationForest :: new () ; forest . register_obligation ("A") ; let TestOutcome { completed : ok , errors : err , .. } = forest . process_obligations (& mut C (| obligation | match * obligation { "A" => ProcessResult :: Changed (thin_vec ! ["A.1" , "A.2" , "A.3"]) , "A.1" | "A.2" | "A.3" => ProcessResult :: Unchanged , _ => unreachable ! () , } , | _ | { } ,)) ; assert_eq ! (ok . len () , 0) ; assert_eq ! (err . len () , 0) ; let errors = forest . to_errors (()) ; assert_eq ! (errors [0] . backtrace , vec ! ["A.1" , "A"]) ; assert_eq ! (errors [1] . backtrace , vec ! ["A.2" , "A"]) ; assert_eq ! (errors [2] . backtrace , vec ! ["A.3" , "A"]) ; assert_eq ! (errors . len () , 3) ; }
};
}

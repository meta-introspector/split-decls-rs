// Generated macro for simultaneous_register_and_error (function)
macro_rules! Depcrate_obligation_forest_testssimultaneous_register_and_error {
() => {
// Module: crate::obligation_forest::tests
// Provides: {"simultaneous_register_and_error"}
// Dependencies: {}
# [test] fn simultaneous_register_and_error () { let mut forest = ObligationForest :: new () ; forest . register_obligation ("A") ; forest . register_obligation ("B") ; let TestOutcome { completed : ok , errors : err , .. } = forest . process_obligations (& mut C (| obligation | match * obligation { "A" => ProcessResult :: Error ("An error") , "B" => ProcessResult :: Changed (thin_vec ! ["A"]) , _ => unreachable ! () , } , | _ | { } ,)) ; assert_eq ! (ok . len () , 0) ; assert_eq ! (err , vec ! [super :: Error { error : "An error" , backtrace : vec ! ["A"] }]) ; let mut forest = ObligationForest :: new () ; forest . register_obligation ("B") ; forest . register_obligation ("A") ; let TestOutcome { completed : ok , errors : err , .. } = forest . process_obligations (& mut C (| obligation | match * obligation { "A" => ProcessResult :: Error ("An error") , "B" => ProcessResult :: Changed (thin_vec ! ["A"]) , _ => unreachable ! () , } , | _ | { } ,)) ; assert_eq ! (ok . len () , 0) ; assert_eq ! (err , vec ! [super :: Error { error : "An error" , backtrace : vec ! ["A"] }]) ; }
};
}

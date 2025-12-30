// Generated macro for tests_std (module)
macro_rules! Depcrate_errortests_std {
() => {
// Module: crate::error
// Provides: {"tests_std"}
// Dependencies: {}
# [cfg (all (feature = "std" , test))] mod tests_std { use crate :: Parser ; # [derive (Clone , PartialEq , Debug)] struct CloneOnly { s : String , } # [test] fn parse_clone_but_not_copy () { let input = & [CloneOnly { s : "x" . to_string () } , CloneOnly { s : "y" . to_string () } ,] [..] ; let result = crate :: parser :: range :: take_while (| c : CloneOnly | c . s == "x") . parse (input) ; assert_eq ! (result , Ok ((& [CloneOnly { s : "x" . to_string () }] [..] , & [CloneOnly { s : "y" . to_string () }] [..]))) ; } }
};
}

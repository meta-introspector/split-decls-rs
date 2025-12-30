// Generated macro for test_some_labelled (function)
macro_rules! Depcrate_teststest_some_labelled {
() => {
// Module: crate::tests
// Provides: {"test_some_labelled"}
// Dependencies: {}
# [test] fn test_some_labelled () { let labels : Trivial = SomeNodesLabelled (vec ! [Some ("A") , None]) ; let styles = Some (vec ! [Style :: None , Style :: Dotted]) ; let result = test_input (LabelledGraph :: new ("test_some_labelled" , labels , vec ! [edge (0 , 1 , "A-1" , Style :: None)] , styles ,)) ; assert_eq ! (result . unwrap () , r#"digraph test_some_labelled {
    N0[label="A"];
    N1[label="N1"][style="dotted"];
    N0 -> N1[label="A-1"];
}
"#) ; }
};
}

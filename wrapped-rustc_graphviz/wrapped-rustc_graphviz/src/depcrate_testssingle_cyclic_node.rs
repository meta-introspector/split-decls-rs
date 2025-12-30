// Generated macro for single_cyclic_node (function)
macro_rules! Depcrate_testssingle_cyclic_node {
() => {
// Module: crate::tests
// Provides: {"single_cyclic_node"}
// Dependencies: {}
# [test] fn single_cyclic_node () { let labels : Trivial = UnlabelledNodes (1) ; let r = test_input (LabelledGraph :: new ("single_cyclic_node" , labels , vec ! [edge (0 , 0 , "E" , Style :: None)] , None ,)) ; assert_eq ! (r . unwrap () , r#"digraph single_cyclic_node {
    N0[label="N0"];
    N0 -> N0[label="E"];
}
"#) ; }
};
}

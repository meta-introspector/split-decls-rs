macro_rules! deps {
    () => {
        LabelledGraph!();
        Trivial!();
    };
}

macro_rules! single_node {
    () => {
        deps!();
        # [test] fn single_node () { let labels : Trivial = UnlabelledNodes (1) ; let r = test_input (LabelledGraph :: new ("single_node" , labels , vec ! [] , None)) ; assert_eq ! (r . unwrap () , r#"digraph single_node {
    N0[label="N0"];
}
"#) ; }
    };
}

single_node!();
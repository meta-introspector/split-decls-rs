macro_rules! deps {
    () => {
        Style!();
        LabelledGraph!();
        Trivial!();
    };
}

macro_rules! single_edge {
    () => {
        deps!();
        # [test] fn single_edge () { let labels : Trivial = UnlabelledNodes (2) ; let result = test_input (LabelledGraph :: new ("single_edge" , labels , vec ! [edge (0 , 1 , "E" , Style :: None)] , None ,)) ; assert_eq ! (result . unwrap () , r#"digraph single_edge {
    N0[label="N0"];
    N1[label="N1"];
    N0 -> N1[label="E"];
}
"#) ; }
    };
}

single_edge!();
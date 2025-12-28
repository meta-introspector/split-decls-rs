macro_rules! deps {
    () => {
        LabelledGraph!();
        Trivial!();
        Style!();
    };
}

macro_rules! single_edge_with_style {
    () => {
        deps!();
        # [test] fn single_edge_with_style () { let labels : Trivial = UnlabelledNodes (2) ; let result = test_input (LabelledGraph :: new ("single_edge" , labels , vec ! [edge (0 , 1 , "E" , Style :: Bold)] , None ,)) ; assert_eq ! (result . unwrap () , r#"digraph single_edge {
    N0[label="N0"];
    N1[label="N1"];
    N0 -> N1[label="E"][style="bold"];
}
"#) ; }
    };
}

single_edge_with_style!()
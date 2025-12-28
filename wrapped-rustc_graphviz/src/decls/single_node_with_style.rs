macro_rules! deps {
    () => {
        Trivial!();
        Style!();
        LabelledGraph!();
    };
}

macro_rules! single_node_with_style {
    () => {
        deps!();
        # [test] fn single_node_with_style () { let labels : Trivial = UnlabelledNodes (1) ; let styles = Some (vec ! [Style :: Dashed]) ; let r = test_input (LabelledGraph :: new ("single_node" , labels , vec ! [] , styles)) ; assert_eq ! (r . unwrap () , r#"digraph single_node {
    N0[label="N0"][style="dashed"];
}
"#) ; }
    };
}

single_node_with_style!()
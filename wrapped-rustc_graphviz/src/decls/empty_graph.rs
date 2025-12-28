macro_rules! deps {
    () => {
        Trivial!();
        LabelledGraph!();
    };
}

macro_rules! empty_graph {
    () => {
        deps!();
        # [test] fn empty_graph () { let labels : Trivial = UnlabelledNodes (0) ; let r = test_input (LabelledGraph :: new ("empty_graph" , labels , vec ! [] , None)) ; assert_eq ! (r . unwrap () , r#"digraph empty_graph {
}
"#) ; }
    };
}

empty_graph!()
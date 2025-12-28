macro_rules! NegativeCycle {
    () => {
        # [doc = " An algorithm error: a cycle of negative weights was found in the graph."] # [derive (Clone , Debug , PartialEq)] pub struct NegativeCycle (pub ()) ;
    };
}

NegativeCycle!();
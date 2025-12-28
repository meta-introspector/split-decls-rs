macro_rules! Normal {
    () => {
        # [doc = " In normal mode, a `R1: R2` constraint results in an edge `R1 ->"] # [doc = " R2`. This is what we use when constructing the SCCs for"] # [doc = " inference. This is because we compute the value of R1 by union'ing"] # [doc = " all the things that it relies on."] # [derive (Copy , Clone , Debug)] pub (crate) struct Normal ;
    };
}

Normal!();
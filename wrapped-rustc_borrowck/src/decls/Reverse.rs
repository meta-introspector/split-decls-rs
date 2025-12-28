macro_rules! Reverse {
    () => {
        # [doc = " In reverse mode, a `R1: R2` constraint results in an edge `R2 ->"] # [doc = " R1`. We use this for optimizing liveness computation, because then"] # [doc = " we wish to iterate from a region (e.g., R2) to all the regions"] # [doc = " that will outlive it (e.g., R1)."] # [derive (Copy , Clone , Debug)] pub (crate) struct Reverse ;
    };
}

Reverse!();
macro_rules! Sample {
    () => {
        # [doc = " A collection of data points drawn from a population"] # [doc = ""] # [doc = " Invariants:"] # [doc = ""] # [doc = " - The sample contains at least 2 data points"] # [doc = " - The sample contains no `NaN`s"] # [repr (transparent)] pub struct Sample < A > ([A]) ;
    };
}

Sample!();
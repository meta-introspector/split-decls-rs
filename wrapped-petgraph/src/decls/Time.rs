macro_rules! Time {
    () => {
        # [doc = " Strictly monotonically increasing event time for a depth first search."] # [derive (Copy , Clone , Debug , PartialEq , PartialOrd , Eq , Ord , Default , Hash)] pub struct Time (pub usize) ;
    };
}

Time!();
macro_rules! deps {
    () => {
        IndexType!();
    };
}

macro_rules! WSuc {
    () => {
        deps!();
        # [doc = " Weighted successor"] # [derive (Clone , Debug , Hash , PartialEq , Eq , PartialOrd , Ord)] struct WSuc < E , Ix : IndexType > { # [doc = " Index of the successor."] suc : Ix , # [doc = " Weight of the edge to `suc`."] weight : E , }
    };
}

WSuc!();
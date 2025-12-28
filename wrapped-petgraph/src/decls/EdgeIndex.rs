macro_rules! deps {
    () => {
        DefaultIx!();
        Edge!();
    };
}

macro_rules! EdgeIndex {
    () => {
        deps!();
        # [doc = " Edge identifier."] # [derive (Copy , Clone , Default , PartialEq , PartialOrd , Eq , Ord , Hash)] pub struct EdgeIndex < Ix = DefaultIx > (Ix) ;
    };
}

EdgeIndex!();
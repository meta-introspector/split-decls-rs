macro_rules! deps {
    () => {
        StackVec!();
    };
}

macro_rules! VecType {
    () => {
        deps!();
        # [cfg (not (feature = "alloc"))] pub type VecType = StackVec ;
    };
}

VecType!();
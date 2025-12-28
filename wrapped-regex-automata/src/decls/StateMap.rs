macro_rules! deps {
    () => {
        LazyStateID!();
        State!();
    };
}

macro_rules! StateMap {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] type StateMap = alloc :: collections :: BTreeMap < State , LazyStateID > ;
    };
}

StateMap!();
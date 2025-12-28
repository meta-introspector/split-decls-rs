macro_rules! deps {
    () => {
        State!();
        LazyStateID!();
    };
}

macro_rules! StateMap {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] type StateMap = alloc :: collections :: BTreeMap < State , LazyStateID > ;
    };
}

StateMap!()
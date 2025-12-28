macro_rules! deps {
    () => {
        StateID!();
    };
}

macro_rules! NextIter {
    () => {
        deps!();
        # [doc = " The next state (and its corresponding transition) that we want to visit"] # [doc = " during iteration in lexicographic order."] # [derive (Clone , Debug)] struct NextIter { state_id : StateID , tidx : usize , }
    };
}

NextIter!()
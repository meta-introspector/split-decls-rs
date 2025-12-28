macro_rules! deps {
    () => {
        StateID!();
    };
}

macro_rules! macro_430 {
    () => {
        deps!();
        index_type_impls ! (StateID , StateIDError , StateIDIter , WithStateIDIter) ;
    };
}

macro_430!()
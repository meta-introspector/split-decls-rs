macro_rules! deps {
    () => {
        Transition!();
    };
}

macro_rules! SparseTransitionIter {
    () => {
        deps!();
        # [doc = " An iterator over groups of consecutive equivalent transitions in a single"] # [doc = " state."] # [derive (Debug)] struct SparseTransitionIter < 'a > { it : core :: iter :: Enumerate < core :: slice :: Iter < 'a , Transition > > , cur : Option < (u8 , u8 , Transition) > , }
    };
}

SparseTransitionIter!()
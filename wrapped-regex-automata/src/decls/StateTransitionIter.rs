macro_rules! deps {
    () => {
        DFA!();
        StateID!();
    };
}

macro_rules! StateTransitionIter {
    () => {
        deps!();
        # [doc = " An iterator over all transitions in a single DFA state. This yields"] # [doc = " a number of transitions equivalent to the alphabet length of the"] # [doc = " corresponding DFA."] # [doc = ""] # [doc = " Each transition is represented by a tuple. The first element is the input"] # [doc = " byte for that transition and the second element is the transition itself."] # [derive (Debug)] pub (crate) struct StateTransitionIter < 'a > { len : usize , it : iter :: Enumerate < slice :: Iter < 'a , StateID > > , }
    };
}

StateTransitionIter!()
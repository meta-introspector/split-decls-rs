macro_rules! deps {
    () => {
        Transitions!();
        DFA!();
    };
}

macro_rules! StateIter {
    () => {
        deps!();
        # [doc = " An iterator over all states in a sparse DFA."] # [doc = ""] # [doc = " This iterator yields tuples, where the first element is the state ID and"] # [doc = " the second element is the state itself."] struct StateIter < 'a , T > { trans : & 'a Transitions < T > , id : usize , }
    };
}

StateIter!();
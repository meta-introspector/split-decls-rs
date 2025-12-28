macro_rules! deps {
    () => {
        StartTable!();
        DFA!();
    };
}

macro_rules! StartStateIter {
    () => {
        deps!();
        # [doc = " An iterator over all state state IDs in a sparse DFA."] struct StartStateIter < 'a , T > { st : & 'a StartTable < T > , i : usize , }
    };
}

StartStateIter!()
macro_rules! deps {
    () => {
        StateID!();
    };
}

macro_rules! ROOT {
    () => {
        deps!();
        # [doc = " The root state of the trie."] const ROOT : StateID = StateID :: new_unchecked (1) ;
    };
}

ROOT!();
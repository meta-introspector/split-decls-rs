macro_rules! State {
    () => {
        # [doc = " A single state in a trie. Uses a sparse representation for its transitions."] # [derive (Debug , Default)] struct State { # [doc = " Sparse representation of the transitions out of this state. Transitions"] # [doc = " are sorted by byte. There is at most one such transition for any"] # [doc = " particular byte."] trans : Vec < (u8 , usize) > , }
    };
}

State!()
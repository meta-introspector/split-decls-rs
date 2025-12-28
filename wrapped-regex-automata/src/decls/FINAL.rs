macro_rules! deps {
    () => {
        StateID!();
    };
}

macro_rules! FINAL {
    () => {
        deps!();
        # [doc = " There is only one final state in this trie. Every sequence of byte ranges"] # [doc = " added shares the same final state."] const FINAL : StateID = StateID :: ZERO ;
    };
}

FINAL!()
macro_rules! deps {
    () => {
        DFA!();
    };
}

macro_rules! MatchStates {
    () => {
        deps!();
        # [doc = " This type represents that patterns that should be reported whenever a DFA"] # [doc = " enters a match state. This structure exists to support DFAs that search for"] # [doc = " matches for multiple regexes."] # [doc = ""] # [doc = " This structure relies on the fact that all match states in a DFA occur"] # [doc = " contiguously in the DFA's transition table. (See dfa/special.rs for a more"] # [doc = " detailed breakdown of the representation.) Namely, when a match occurs, we"] # [doc = " know its state ID. Since we know the start and end of the contiguous region"] # [doc = " of match states, we can use that to compute the position at which the match"] # [doc = " state occurs. That in turn is used as an offset into this structure."] # [derive (Clone , Debug)] struct MatchStates < T > { # [doc = " Slices is a flattened sequence of pairs, where each pair points to a"] # [doc = " sub-slice of pattern_ids. The first element of the pair is an offset"] # [doc = " into pattern_ids and the second element of the pair is the number"] # [doc = " of 32-bit pattern IDs starting at that position. That is, each pair"] # [doc = " corresponds to a single DFA match state and its corresponding match"] # [doc = " IDs. The number of pairs always corresponds to the number of distinct"] # [doc = " DFA match states."] # [doc = ""] # [doc = " In practice, T is either Vec<u32> or &[u32]."] slices : T , # [doc = " A flattened sequence of pattern IDs for each DFA match state. The only"] # [doc = " way to correctly read this sequence is indirectly via `slices`."] # [doc = ""] # [doc = " In practice, T is either Vec<u32> or &[u32]."] pattern_ids : T , # [doc = " The total number of unique patterns represented by these match states."] pattern_len : usize , }
    };
}

MatchStates!();
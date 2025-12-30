// Generated macro for TRANS_CLASSES (const)
macro_rules! Depcrate_readerTRANS_CLASSES {
() => {
// Module: crate::reader
// Provides: {"TRANS_CLASSES"}
// Dependencies: {}
# [doc = " The number of slots in the DFA transition table."] # [doc = ""] # [doc = " This number is computed by multiplying the maximum number of transition"] # [doc = " classes (7) by the total number of NFA states that are used in the DFA"] # [doc = " (10)."] # [doc = ""] # [doc = " The number of transition classes is determined by an equivalence class of"] # [doc = " bytes, where every byte in the same equivalence classes is"] # [doc = " indistinguishable from any other byte with respect to the DFA. For example,"] # [doc = " if neither `a` nor `b` are specifed as a delimiter/quote/terminator/escape,"] # [doc = " then the DFA will never discriminate between `a` or `b`, so they can"] # [doc = " effectively be treated as identical. This reduces storage space"] # [doc = " substantially."] # [doc = ""] # [doc = " The total number of NFA states (13) is greater than the total number of"] # [doc = " NFA states that are in the DFA. In particular, any NFA state that can only"] # [doc = " be reached by epsilon transitions will never have explicit usage in the"] # [doc = " DFA."] const TRANS_CLASSES : usize = 7 ;
};
}

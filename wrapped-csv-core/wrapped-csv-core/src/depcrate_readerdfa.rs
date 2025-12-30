// Generated macro for Dfa (struct)
macro_rules! Depcrate_readerDfa {
() => {
// Module: crate::reader
// Provides: {"Dfa"}
// Dependencies: {}
# [doc = " A representation of a DFA."] # [doc = ""] # [doc = " For the most part, this is a transition table, but various optimizations"] # [doc = " have been applied to reduce its memory footprint."] struct Dfa { # [doc = " The core transition table. Each row corresponds to the transitions for"] # [doc = " each input equivalence class. (Input bytes are mapped to their"] # [doc = " corresponding equivalence class with the `classes` map.)"] # [doc = ""] # [doc = " DFA states are represented as an index corresponding to the start of"] # [doc = " its row in this table."] trans : [DfaState ; TRANS_SIZE] , # [doc = " A table with the same layout as `trans`, except its values indicate"] # [doc = " whether a particular `(state, equivalence class)` pair should emit an"] # [doc = " output byte."] has_output : [bool ; TRANS_SIZE] , # [doc = " A map from input byte to equivalence class."] # [doc = ""] # [doc = " This is responsible for reducing the effective alphabet size from"] # [doc = " 256 to `TRANS_CLASSES`."] classes : DfaClasses , # [doc = " The DFA state corresponding to being inside an unquoted field."] in_field : DfaState , # [doc = " The DFA state corresponding to being inside an quoted field."] in_quoted : DfaState , # [doc = " The minimum DFA state that indicates a field has been parsed. All DFA"] # [doc = " states greater than this are also final-field states."] final_field : DfaState , # [doc = " The minimum DFA state that indicates a record has been parsed. All DFA"] # [doc = " states greater than this are also final-record states."] final_record : DfaState , }
};
}

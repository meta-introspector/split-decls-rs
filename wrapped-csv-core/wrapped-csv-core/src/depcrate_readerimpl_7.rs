// Generated macro for impl_7 (impl)
macro_rules! Depcrate_readerimpl_7 {
() => {
// Module: crate::reader
// Provides: {"impl_7"}
// Dependencies: {}
impl Default for Reader { fn default () -> Reader { Reader { dfa : Dfa :: new () , dfa_state : DfaState :: start () , nfa_state : NfaState :: StartRecord , delimiter : b',' , term : Terminator :: default () , quote : b'"' , escape : None , double_quote : true , comment : None , quoting : true , use_nfa : false , line : 1 , has_read : false , output_pos : 0 , } } }
};
}

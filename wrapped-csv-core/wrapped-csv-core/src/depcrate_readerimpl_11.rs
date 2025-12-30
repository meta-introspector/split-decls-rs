// Generated macro for impl_11 (impl)
macro_rules! Depcrate_readerimpl_11 {
() => {
// Module: crate::reader
// Provides: {"impl_11"}
// Dependencies: {}
impl ReadFieldResult { fn from_nfa (state : NfaState , inpdone : bool , outdone : bool ,) -> ReadFieldResult { match state { NfaState :: End => ReadFieldResult :: End , NfaState :: EndRecord | NfaState :: CRLF => { ReadFieldResult :: Field { record_end : true } } NfaState :: EndFieldDelim => { ReadFieldResult :: Field { record_end : false } } _ => { assert ! (! state . is_field_final ()) ; if ! inpdone && outdone { ReadFieldResult :: OutputFull } else { ReadFieldResult :: InputEmpty } } } } }
};
}

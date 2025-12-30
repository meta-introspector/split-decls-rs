// Generated macro for impl_14 (impl)
macro_rules! Depcrate_readerimpl_14 {
() => {
// Module: crate::reader
// Provides: {"impl_14"}
// Dependencies: {}
impl ReadRecordResult { fn is_record (& self) -> bool { * self == ReadRecordResult :: Record } fn from_nfa (state : NfaState , inpdone : bool , outdone : bool , endsdone : bool ,) -> ReadRecordResult { match state { NfaState :: End => ReadRecordResult :: End , NfaState :: EndRecord | NfaState :: CRLF => ReadRecordResult :: Record , _ => { assert ! (! state . is_record_final ()) ; if ! inpdone && outdone { ReadRecordResult :: OutputFull } else if ! inpdone && endsdone { ReadRecordResult :: OutputEndsFull } else { ReadRecordResult :: InputEmpty } } } } }
};
}

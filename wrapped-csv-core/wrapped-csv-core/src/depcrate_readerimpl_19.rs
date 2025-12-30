// Generated macro for impl_19 (impl)
macro_rules! Depcrate_readerimpl_19 {
() => {
// Module: crate::reader
// Provides: {"impl_19"}
// Dependencies: {}
impl NfaState { # [doc = " Returns true if this state indicates that a field has been parsed."] fn is_field_final (& self) -> bool { matches ! (* self , NfaState :: End | NfaState :: EndRecord | NfaState :: CRLF | NfaState :: EndFieldDelim) } # [doc = " Returns true if this state indicates that a record has been parsed."] fn is_record_final (& self) -> bool { matches ! (* self , NfaState :: End | NfaState :: EndRecord | NfaState :: CRLF) } }
};
}

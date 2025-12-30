// Generated macro for PushEntryList (function)
macro_rules! Depcrate_ntrtlPushEntryList {
() => {
// Module: crate::ntrtl
// Provides: {"PushEntryList"}
// Dependencies: {}
# [inline] pub fn PushEntryList (ListHead : & mut SINGLE_LIST_ENTRY , Entry : & mut SINGLE_LIST_ENTRY) { Entry . Next = ListHead . Next ; ListHead . Next = Entry ; }
};
}

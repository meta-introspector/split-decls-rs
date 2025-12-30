// Generated macro for PopEntryList (function)
macro_rules! Depcrate_ntrtlPopEntryList {
() => {
// Module: crate::ntrtl
// Provides: {"PopEntryList"}
// Dependencies: {}
# [inline] pub unsafe fn PopEntryList (ListHead : & mut SINGLE_LIST_ENTRY) -> PSINGLE_LIST_ENTRY { let FirstEntry = ListHead . Next ; if ! FirstEntry . is_null () { ListHead . Next = (* FirstEntry) . Next ; } FirstEntry }
};
}

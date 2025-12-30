// Generated macro for RemoveEntryList (function)
macro_rules! Depcrate_ntrtlRemoveEntryList {
() => {
// Module: crate::ntrtl
// Provides: {"RemoveEntryList"}
// Dependencies: {}
# [inline] pub unsafe fn RemoveEntryList (Entry : & mut LIST_ENTRY) -> bool { let (Blink , Flink) = (Entry . Blink , Entry . Flink) ; (* Blink) . Flink = Flink ; (* Flink) . Blink = Blink ; Flink == Blink }
};
}

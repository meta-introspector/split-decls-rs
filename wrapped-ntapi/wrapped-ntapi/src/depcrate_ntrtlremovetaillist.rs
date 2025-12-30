// Generated macro for RemoveTailList (function)
macro_rules! Depcrate_ntrtlRemoveTailList {
() => {
// Module: crate::ntrtl
// Provides: {"RemoveTailList"}
// Dependencies: {}
# [inline] pub unsafe fn RemoveTailList (ListHead : & mut LIST_ENTRY) -> PLIST_ENTRY { let Entry = ListHead . Blink ; let Blink = (* Entry) . Blink ; ListHead . Blink = Blink ; (* Blink) . Flink = ListHead ; Entry }
};
}

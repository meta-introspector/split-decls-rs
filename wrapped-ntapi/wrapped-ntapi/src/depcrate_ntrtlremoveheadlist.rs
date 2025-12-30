// Generated macro for RemoveHeadList (function)
macro_rules! Depcrate_ntrtlRemoveHeadList {
() => {
// Module: crate::ntrtl
// Provides: {"RemoveHeadList"}
// Dependencies: {}
# [inline] pub unsafe fn RemoveHeadList (ListHead : & mut LIST_ENTRY) -> PLIST_ENTRY { let Entry = ListHead . Flink ; let Flink = (* Entry) . Flink ; ListHead . Flink = Flink ; (* Flink) . Blink = ListHead ; Entry }
};
}

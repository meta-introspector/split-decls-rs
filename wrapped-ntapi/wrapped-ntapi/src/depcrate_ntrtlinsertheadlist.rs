// Generated macro for InsertHeadList (function)
macro_rules! Depcrate_ntrtlInsertHeadList {
() => {
// Module: crate::ntrtl
// Provides: {"InsertHeadList"}
// Dependencies: {}
# [inline] pub unsafe fn InsertHeadList (ListHead : & mut LIST_ENTRY , Entry : & mut LIST_ENTRY) { let Flink = ListHead . Flink ; Entry . Flink = Flink ; Entry . Blink = ListHead ; (* Flink) . Blink = Entry ; ListHead . Flink = Entry ; }
};
}

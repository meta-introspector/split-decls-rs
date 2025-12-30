// Generated macro for InsertTailList (function)
macro_rules! Depcrate_ntrtlInsertTailList {
() => {
// Module: crate::ntrtl
// Provides: {"InsertTailList"}
// Dependencies: {}
# [inline] pub unsafe fn InsertTailList (ListHead : & mut LIST_ENTRY , Entry : & mut LIST_ENTRY) { let Blink = ListHead . Blink ; Entry . Flink = ListHead ; Entry . Blink = Blink ; (* Blink) . Flink = Entry ; ListHead . Blink = Entry ; }
};
}

// Generated macro for AppendTailList (function)
macro_rules! Depcrate_ntrtlAppendTailList {
() => {
// Module: crate::ntrtl
// Provides: {"AppendTailList"}
// Dependencies: {}
# [inline] pub unsafe fn AppendTailList (ListHead : & mut LIST_ENTRY , ListToAppend : & mut LIST_ENTRY) { let ListEnd = ListHead . Blink ; (* ListHead . Blink) . Flink = ListToAppend ; ListHead . Blink = ListToAppend . Blink ; (* ListToAppend . Blink) . Flink = ListHead ; ListToAppend . Blink = ListEnd ; }
};
}

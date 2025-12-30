// Generated macro for InitializeListHead (function)
macro_rules! Depcrate_ntrtlInitializeListHead {
() => {
// Module: crate::ntrtl
// Provides: {"InitializeListHead"}
// Dependencies: {}
# [inline] pub fn InitializeListHead (ListHead : & mut LIST_ENTRY) { ListHead . Flink = ListHead ; ListHead . Blink = ListHead ; }
};
}

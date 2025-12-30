// Generated macro for IsListEmpty (function)
macro_rules! Depcrate_ntrtlIsListEmpty {
() => {
// Module: crate::ntrtl
// Provides: {"IsListEmpty"}
// Dependencies: {}
# [inline] pub fn IsListEmpty (ListHead : & LIST_ENTRY) -> bool { ListHead . Flink as * const _ == ListHead as * const _ }
};
}

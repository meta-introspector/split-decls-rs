// Generated macro for HotData (struct)
macro_rules! Depcrate_baseHotData {
() => {
// Module: crate::base
// Provides: {"HotData"}
// Dependencies: {}
# [doc = " Frequently modified data associated with a skip list."] struct HotData { # [doc = " The seed for random height generation."] seed : AtomicUsize , # [doc = " The number of entries in the skip list."] len : AtomicUsize , # [doc = " Highest tower currently in use. This value is used as a hint for where"] # [doc = " to start lookups and never decreases."] max_height : AtomicUsize , }
};
}

// Generated macro for MemoryProfile (struct)
macro_rules! Depcrate_valgrindMemoryProfile {
() => {
// Module: crate::valgrind
// Provides: {"MemoryProfile"}
// Dependencies: {}
# [doc = " Peak heap usage in bytes, for each side"] # [derive (Copy , Clone)] pub (crate) struct MemoryProfile { pub client : MemoryDetails , pub server : MemoryDetails , }
};
}

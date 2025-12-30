// Generated macro for Decision (enum)
macro_rules! Depcrate_compaction_filterDecision {
() => {
// Module: crate::compaction_filter
// Provides: {"Decision"}
// Dependencies: {}
# [doc = " Decision about how to handle compacting an object"] # [doc = ""] # [doc = " This is returned by a compaction filter callback. Depending"] # [doc = " on the value, the object may be kept, removed, or changed"] # [doc = " in the database during a compaction."] pub enum Decision { # [doc = " Keep the old value"] Keep , # [doc = " Remove the object from the database"] Remove , # [doc = " Change the value for the key"] Change (& 'static [u8]) , }
};
}

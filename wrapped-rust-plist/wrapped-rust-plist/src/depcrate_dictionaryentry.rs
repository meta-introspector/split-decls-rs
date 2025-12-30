// Generated macro for Entry (enum)
macro_rules! Depcrate_dictionaryEntry {
() => {
// Module: crate::dictionary
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " A view into a single entry in a dictionary, which may either be vacant or occupied."] # [doc = " This enum is constructed from the [`entry`] method on [`Dictionary`]."] # [doc = ""] # [doc = " [`entry`]: struct.Dictionary.html#method.entry"] # [doc = " [`Dictionary`]: struct.Dictionary.html"] # [cfg (any (test , feature = "enable_unstable_features_that_may_break_with_minor_version_bumps"))] pub enum Entry < 'a > { # [doc = " A vacant Entry."] Vacant (VacantEntry < 'a >) , # [doc = " An occupied Entry."] Occupied (OccupiedEntry < 'a >) , }
};
}

// Generated macro for Entry (enum)
macro_rules! Depcrate_list_ordered_multimapEntry {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " A view into a single entry in the multimap, which may either be vacant or occupied."] pub enum Entry < 'map , Key , Value , State = RandomState > { # [doc = " An occupied entry associated with one or more values."] Occupied (OccupiedEntry < 'map , Key , Value >) , # [doc = " A vacant entry with no associated values."] Vacant (VacantEntry < 'map , Key , Value , State >) , }
};
}

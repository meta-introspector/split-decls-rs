// Generated macro for OccupiedEntry (struct)
macro_rules! Depcrate_list_ordered_multimapOccupiedEntry {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"OccupiedEntry"}
// Dependencies: {}
# [doc = " A view into an occupied entry in the multimap."] pub struct OccupiedEntry < 'map , Key , Value > { entry : RawOccupiedEntryMut < 'map , Index < Key > , MapEntry < Key , Value > , DummyState > , keys : & 'map mut VecList < Key > , values : & 'map mut VecList < ValueEntry < Key , Value > > , }
};
}

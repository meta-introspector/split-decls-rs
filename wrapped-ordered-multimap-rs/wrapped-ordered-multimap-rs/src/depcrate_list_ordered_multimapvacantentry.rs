// Generated macro for VacantEntry (struct)
macro_rules! Depcrate_list_ordered_multimapVacantEntry {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"VacantEntry"}
// Dependencies: {}
# [doc = " A view into a vacant entry in the multimap."] pub struct VacantEntry < 'map , Key , Value , State = RandomState > { # [doc = " The builder hasher for the map, kept separately for mutability concerns."] build_hasher : & 'map State , # [doc = " The hash of the key for the entry."] hash : u64 , # [doc = " The key for this entry for when it is to be inserted into the map."] key : Key , keys : & 'map mut VecList < Key > , # [doc = " Reference to the multimap."] map : & 'map mut HashMap < Index < Key > , MapEntry < Key , Value > , DummyState > , values : & 'map mut VecList < ValueEntry < Key , Value > > , }
};
}

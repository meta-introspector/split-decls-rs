// Generated macro for KeyValues (struct)
macro_rules! Depcrate_list_ordered_multimapKeyValues {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"KeyValues"}
// Dependencies: {}
# [doc = " An iterator that yields immutable references to all keys and their value iterators. The order of the yielded items"] # [doc = " is always in the order the keys were first inserted."] pub struct KeyValues < 'map , Key , Value , State = RandomState > { # [doc = " The builder hasher for the map, kept separately for mutability concerns."] build_hasher : & 'map State , keys : & 'map VecList < Key > , # [doc = " The iterator over the list of all values. This is ordered by time of insertion."] iter : VecListIter < 'map , Key > , # [doc = " The internal mapping from key hashes to associated value indices."] map : & 'map HashMap < Index < Key > , MapEntry < Key , Value > , DummyState > , # [doc = " The list of the values in the map. This is ordered by time of insertion."] values : & 'map VecList < ValueEntry < Key , Value > > , }
};
}

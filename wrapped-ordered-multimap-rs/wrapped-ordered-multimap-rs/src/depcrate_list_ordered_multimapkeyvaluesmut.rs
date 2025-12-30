// Generated macro for KeyValuesMut (struct)
macro_rules! Depcrate_list_ordered_multimapKeyValuesMut {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"KeyValuesMut"}
// Dependencies: {}
# [doc = " An iterator that yields mutable references to all keys and their value iterators. The order of the yielded items is"] # [doc = " always in the order the keys were first inserted."] pub struct KeyValuesMut < 'map , Key , Value , State = RandomState > { # [doc = " The builder hasher for the map, kept separately for mutability concerns."] build_hasher : & 'map State , keys : & 'map VecList < Key > , # [doc = " The iterator over the list of all values. This is ordered by time of insertion."] iter : VecListIter < 'map , Key > , # [doc = " The internal mapping from key hashes to associated value indices."] map : & 'map HashMap < Index < Key > , MapEntry < Key , Value > , DummyState > , # [doc = " The list of the values in the map. This is ordered by time of insertion."] values : * mut VecList < ValueEntry < Key , Value > > , }
};
}

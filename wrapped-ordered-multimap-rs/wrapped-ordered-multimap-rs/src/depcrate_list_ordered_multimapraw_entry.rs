// Generated macro for raw_entry (function)
macro_rules! Depcrate_list_ordered_multimapraw_entry {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"raw_entry"}
// Dependencies: {}
# [must_use] fn raw_entry < 'map , Key , KeyQuery , Value , State > (keys : & VecList < Key > , map : & 'map HashMap < Index < Key > , MapEntry < Key , Value > , State > , hash : u64 , key : & KeyQuery ,) -> Option < (& 'map Index < Key > , & 'map MapEntry < Key , Value >) > where Key : Borrow < KeyQuery > + Eq + Hash , KeyQuery : ? Sized + Eq + Hash , State : BuildHasher , { map . raw_entry () . from_hash (hash , | & key_index | { let existing_key = keys . get (key_index) . unwrap () ; key == existing_key . borrow () }) }
};
}

// Generated macro for raw_entry_mut (function)
macro_rules! Depcrate_list_ordered_multimapraw_entry_mut {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"raw_entry_mut"}
// Dependencies: {}
# [must_use] fn raw_entry_mut < 'map , Key , KeyQuery , Value , State > (keys : & VecList < Key > , map : & 'map mut HashMap < Index < Key > , MapEntry < Key , Value > , State > , hash : u64 , key : & KeyQuery ,) -> RawEntryMut < 'map , Index < Key > , MapEntry < Key , Value > , State > where Key : Borrow < KeyQuery > + Eq + Hash , KeyQuery : ? Sized + Eq + Hash , State : BuildHasher , { map . raw_entry_mut () . from_hash (hash , | & key_index | { let existing_key = keys . get (key_index) . unwrap () ; key == existing_key . borrow () }) }
};
}

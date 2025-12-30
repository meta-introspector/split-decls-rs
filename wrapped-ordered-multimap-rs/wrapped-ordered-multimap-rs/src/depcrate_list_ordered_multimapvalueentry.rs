// Generated macro for ValueEntry (struct)
macro_rules! Depcrate_list_ordered_multimapValueEntry {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"ValueEntry"}
// Dependencies: {}
# [doc = " The value entry that is contained within the internal values list."] # [derive (Clone)] pub (crate) struct ValueEntry < Key , Value > { # [doc = " The index of the key in the key list for this entry."] key_index : Index < Key > , # [doc = " The index of the next value with the same key."] next_index : Option < Index < ValueEntry < Key , Value > > > , # [doc = " The index of the previous value with the same key."] previous_index : Option < Index < ValueEntry < Key , Value > > > , # [doc = " The actual value stored in this entry."] value : Value , }
};
}

// Generated macro for impl_38 (impl)
macro_rules! Depcrate_baseimpl_38 {
() => {
// Module: crate::base
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'a : 'g , 'g , K : 'a , V : 'a > Entry < 'a , 'g , K , V > { # [doc = " Returns `true` if the entry is removed from the skip list."] pub fn is_removed (& self) -> bool { self . node . is_removed () } # [doc = " Returns a reference to the key."] pub fn key (& self) -> & 'g K { & self . node . key } # [doc = " Returns a reference to the value."] pub fn value (& self) -> & 'g V { & self . node . value } # [doc = " Returns a reference to the parent `SkipList`"] pub fn skiplist (& self) -> & 'a SkipList < K , V > { self . parent } # [doc = " Attempts to pin the entry with a reference count, ensuring that it"] # [doc = " remains accessible even after the `Guard` is dropped."] # [doc = ""] # [doc = " This method may return `None` if the reference count is already 0 and"] # [doc = " the node has been queued for deletion."] pub fn pin (& self) -> Option < RefEntry < 'a , K , V > > { unsafe { RefEntry :: try_acquire (self . parent , self . node) } } }
};
}

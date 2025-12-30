// Generated macro for impl_44 (impl)
macro_rules! Depcrate_baseimpl_44 {
() => {
// Module: crate::base
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > RefEntry < 'a , K , V > { # [doc = " Returns `true` if the entry is removed from the skip list."] pub fn is_removed (& self) -> bool { self . node . is_removed () } # [doc = " Returns a reference to the key."] pub fn key (& self) -> & 'a K { & self . node . key } # [doc = " Returns a reference to the value."] pub fn value (& self) -> & 'a V { & self . node . value } # [doc = " Returns a reference to the parent `SkipList`"] pub fn skiplist (& self) -> & 'a SkipList < K , V > { self . parent } # [doc = " Releases the reference on the entry."] pub fn release (self , guard : & Guard) { self . parent . check_guard (guard) ; unsafe { self . node . decrement (guard) } } # [doc = " Releases the reference of the entry, pinning the thread only when"] # [doc = " the reference count of the node becomes 0."] pub fn release_with_pin < F > (self , pin : F) where F : FnOnce () -> Guard , { unsafe { self . node . decrement_with_pin (self . parent , pin) } } # [doc = " Tries to create a new `RefEntry` by incrementing the reference count of"] # [doc = " a node."] unsafe fn try_acquire (parent : & 'a SkipList < K , V > , node : & Node < K , V > ,) -> Option < RefEntry < 'a , K , V > > { if unsafe { node . try_increment () } { Some (RefEntry { parent , node : unsafe { & * (node as * const _) } , }) } else { None } } }
};
}

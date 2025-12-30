// Generated macro for impl_146 (impl)
macro_rules! Depcrate_linked_hash_setimpl_146 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_146"}
// Dependencies: {}
impl < T , S > PartialEq for LinkedHashSet < T , S > where T : Eq + Hash , S : BuildHasher , { # [inline] fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . iter () . eq (other) } }
};
}

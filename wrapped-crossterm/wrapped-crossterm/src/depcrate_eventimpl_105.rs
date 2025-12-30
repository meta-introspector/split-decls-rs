// Generated macro for impl_105 (impl)
macro_rules! Depcrate_eventimpl_105 {
() => {
// Module: crate::event
// Provides: {"impl_105"}
// Dependencies: {}
impl Hash for KeyEvent { fn hash < H : Hasher > (& self , hash_state : & mut H) { let KeyEvent { code , modifiers , kind , state , } = self . normalize_case () ; code . hash (hash_state) ; modifiers . hash (hash_state) ; kind . hash (hash_state) ; state . hash (hash_state) ; } }
};
}

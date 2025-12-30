// Generated macro for impl_15 (impl)
macro_rules! Depcrate_persisting_hasherimpl_15 {
() => {
// Module: crate::persisting_hasher
// Provides: {"impl_15"}
// Dependencies: {}
impl BuildHasher for PersistingHasherBuilder { type Hasher = PersistingHasher ; fn build_hasher (& self) -> Self :: Hasher { PersistingHasher { hash : self . id , out : self . out . clone () , } } }
};
}

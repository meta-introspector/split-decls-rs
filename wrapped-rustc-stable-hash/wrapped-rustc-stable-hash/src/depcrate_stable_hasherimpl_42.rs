// Generated macro for impl_42 (impl)
macro_rules! Depcrate_stable_hasherimpl_42 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_42"}
// Dependencies: {}
impl < H : ExtendedHasher + fmt :: Debug > fmt :: Debug for StableHasher < H > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:?}" , self . state) } }
};
}

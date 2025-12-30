// Generated macro for impl_54 (impl)
macro_rules! Depcrate_providerimpl_54 {
() => {
// Module: crate::provider
// Provides: {"impl_54"}
// Dependencies: {}
impl < V : VarULE + fmt :: Debug + ? Sized > fmt :: Debug for PluralElementsPackedULE < V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let unpacked = self . as_parts () ; f . debug_struct ("PluralElementsPackedULE") . field ("parts" , & unpacked) . field ("bytes" , & & self . bytes) . finish () } }
};
}

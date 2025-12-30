// Generated macro for impl_172 (impl)
macro_rules! Depcrate_setimpl_172 {
() => {
// Module: crate::set
// Provides: {"impl_172"}
// Dependencies: {}
impl < K : Eq + Hash + fmt :: Debug , S : BuildHasher + Clone > fmt :: Debug for DashSet < K , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . inner , f) } }
};
}

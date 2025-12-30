// Generated macro for impl_4 (impl)
macro_rules! Depcrate_arbitraryimpl_4 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_4"}
// Dependencies: {}
impl < 'a , K , V , S > Arbitrary < 'a > for crate :: DashMap < K , V , S > where K : Eq + std :: hash :: Hash + Arbitrary < 'a > , V : Arbitrary < 'a > , S : Default + BuildHasher + Clone , { fn arbitrary (u : & mut Unstructured < 'a >) -> arbitrary :: Result < Self > { u . arbitrary_iter () ? . collect () } }
};
}

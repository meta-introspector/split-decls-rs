// Generated macro for impl_609 (impl)
macro_rules! Depcrate_serimpl_609 {
() => {
// Module: crate::ser
// Provides: {"impl_609"}
// Dependencies: {}
impl < 'de , A : Deserialize < 'de > + Hash + Eq + Clone , S : BuildHasher + Default > Deserialize < 'de > for HashSet < A , S > { fn deserialize < D > (des : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { des . deserialize_seq (SeqVisitor :: new ()) } }
};
}

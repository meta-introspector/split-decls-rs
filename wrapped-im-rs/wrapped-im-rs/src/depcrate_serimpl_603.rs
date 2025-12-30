// Generated macro for impl_603 (impl)
macro_rules! Depcrate_serimpl_603 {
() => {
// Module: crate::ser
// Provides: {"impl_603"}
// Dependencies: {}
impl < 'de , A : Deserialize < 'de > + Ord + Clone > Deserialize < 'de > for OrdSet < A > { fn deserialize < D > (des : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { des . deserialize_seq (SeqVisitor :: new ()) } }
};
}

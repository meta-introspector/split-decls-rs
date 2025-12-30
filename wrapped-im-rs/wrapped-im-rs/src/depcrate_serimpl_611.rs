// Generated macro for impl_611 (impl)
macro_rules! Depcrate_serimpl_611 {
() => {
// Module: crate::ser
// Provides: {"impl_611"}
// Dependencies: {}
impl < 'de , A : Clone + Deserialize < 'de > > Deserialize < 'de > for Vector < A > { fn deserialize < D > (des : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { des . deserialize_seq (SeqVisitor :: < 'de , Vector < A > , A > :: new ()) } }
};
}

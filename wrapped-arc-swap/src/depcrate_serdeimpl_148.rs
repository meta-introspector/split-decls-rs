// Generated macro for impl_148 (impl)
macro_rules! Depcrate_serdeimpl_148 {
() => {
// Module: crate::serde
// Provides: {"impl_148"}
// Dependencies: {}
impl < T , S > Serialize for ArcSwapAny < T , S > where T : RefCnt + Serialize , S : Strategy < T > , { fn serialize < Ser : Serializer > (& self , serializer : Ser) -> Result < Ser :: Ok , Ser :: Error > { self . load () . serialize (serializer) } }
};
}

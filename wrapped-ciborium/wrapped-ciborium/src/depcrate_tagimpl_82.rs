// Generated macro for impl_82 (impl)
macro_rules! Depcrate_tagimpl_82 {
() => {
// Module: crate::tag
// Provides: {"impl_82"}
// Dependencies: {}
impl < V : Serialize > Serialize for RequireAny < V > { # [inline] fn serialize < S : ser :: Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { Internal :: Tagged (self . 0 , & self . 1) . serialize (serializer) } }
};
}

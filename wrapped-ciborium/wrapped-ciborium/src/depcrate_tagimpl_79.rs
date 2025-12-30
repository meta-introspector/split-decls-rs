// Generated macro for impl_79 (impl)
macro_rules! Depcrate_tagimpl_79 {
() => {
// Module: crate::tag
// Provides: {"impl_79"}
// Dependencies: {}
impl < V : Serialize , const TAG : u64 > Serialize for AllowExact < V , TAG > { # [inline] fn serialize < S : ser :: Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { Internal :: Tagged (TAG , & self . 0) . serialize (serializer) } }
};
}

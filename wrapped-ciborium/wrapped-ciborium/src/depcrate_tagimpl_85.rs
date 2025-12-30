// Generated macro for impl_85 (impl)
macro_rules! Depcrate_tagimpl_85 {
() => {
// Module: crate::tag
// Provides: {"impl_85"}
// Dependencies: {}
impl < V : Serialize , const TAG : u64 > Serialize for RequireExact < V , TAG > { # [inline] fn serialize < S : ser :: Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { Internal :: Tagged (TAG , & self . 0) . serialize (serializer) } }
};
}

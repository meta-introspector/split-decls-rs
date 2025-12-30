// Generated macro for impl_76 (impl)
macro_rules! Depcrate_tagimpl_76 {
() => {
// Module: crate::tag
// Provides: {"impl_76"}
// Dependencies: {}
impl < V : Serialize > Serialize for AllowAny < V > { # [inline] fn serialize < S : ser :: Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { match self . 0 { Some (tag) => Internal :: Tagged (tag , & self . 1) . serialize (serializer) , None => Internal :: Untagged (& self . 1) . serialize (serializer) , } } }
};
}

// Generated macro for impl_78 (impl)
macro_rules! Depcrate_tagimpl_78 {
() => {
// Module: crate::tag
// Provides: {"impl_78"}
// Dependencies: {}
impl < 'de , V : Deserialize < 'de > , const TAG : u64 > Deserialize < 'de > for AllowExact < V , TAG > { # [inline] fn deserialize < D : de :: Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { match Internal :: deserialize (deserializer) ? { Internal :: Tagged (t , v) if t == TAG => Ok (AllowExact (v)) , Internal :: Untagged (v) => Ok (AllowExact (v)) , _ => Err (de :: Error :: custom ("required tag not found")) , } } }
};
}

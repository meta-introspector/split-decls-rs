// Generated macro for impl_84 (impl)
macro_rules! Depcrate_tagimpl_84 {
() => {
// Module: crate::tag
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'de , V : Deserialize < 'de > , const TAG : u64 > Deserialize < 'de > for RequireExact < V , TAG > { # [inline] fn deserialize < D : de :: Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { match Internal :: deserialize (deserializer) ? { Internal :: Tagged (t , v) if t == TAG => Ok (RequireExact (v)) , _ => Err (de :: Error :: custom ("required tag not found")) , } } }
};
}

// Generated macro for impl_75 (impl)
macro_rules! Depcrate_tagimpl_75 {
() => {
// Module: crate::tag
// Provides: {"impl_75"}
// Dependencies: {}
impl < 'de , V : Deserialize < 'de > > Deserialize < 'de > for AllowAny < V > { # [inline] fn deserialize < D : de :: Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { match Internal :: deserialize (deserializer) ? { Internal :: Tagged (t , v) => Ok (AllowAny (Some (t) , v)) , Internal :: Untagged (v) => Ok (AllowAny (None , v)) , } } }
};
}

// Generated macro for impl_81 (impl)
macro_rules! Depcrate_tagimpl_81 {
() => {
// Module: crate::tag
// Provides: {"impl_81"}
// Dependencies: {}
impl < 'de , V : Deserialize < 'de > > Deserialize < 'de > for RequireAny < V > { # [inline] fn deserialize < D : de :: Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { match Internal :: deserialize (deserializer) ? { Internal :: Tagged (t , v) => Ok (RequireAny (t , v)) , _ => Err (de :: Error :: custom ("required tag not found")) , } } }
};
}

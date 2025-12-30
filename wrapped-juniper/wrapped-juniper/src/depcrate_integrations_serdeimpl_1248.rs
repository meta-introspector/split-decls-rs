// Generated macro for impl_1248 (impl)
macro_rules! Depcrate_integrations_serdeimpl_1248 {
() => {
// Module: crate::integrations::serde
// Provides: {"impl_1248"}
// Dependencies: {}
impl < T : Serialize > Serialize for Value < T > { fn serialize < S : Serializer > (& self , ser : S) -> Result < S :: Ok , S :: Error > { match self { Self :: Null => ser . serialize_unit () , Self :: Scalar (s) => s . serialize (ser) , Self :: List (l) => l . serialize (ser) , Self :: Object (o) => o . serialize (ser) , } } }
};
}

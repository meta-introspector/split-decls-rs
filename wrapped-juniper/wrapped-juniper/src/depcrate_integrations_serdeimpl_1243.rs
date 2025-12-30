// Generated macro for impl_1243 (impl)
macro_rules! Depcrate_integrations_serdeimpl_1243 {
() => {
// Module: crate::integrations::serde
// Provides: {"impl_1243"}
// Dependencies: {}
impl < T : Serialize > Serialize for InputValue < T > { fn serialize < S : Serializer > (& self , ser : S) -> Result < S :: Ok , S :: Error > { match self { Self :: Null | Self :: Variable (_) => ser . serialize_unit () , Self :: Scalar (s) => s . serialize (ser) , Self :: Enum (e) => ser . serialize_str (e) , Self :: List (l) => l . iter () . map (| x | & x . item) . collect :: < Vec < _ > > () . serialize (ser) , Self :: Object (o) => o . iter () . map (| (k , v) | (k . item . as_str () , & v . item)) . collect :: < IndexMap < _ , _ > > () . serialize (ser) , } } }
};
}

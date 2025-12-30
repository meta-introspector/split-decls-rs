// Generated macro for impl_1128 (impl)
macro_rules! Depcrate_types_maybe_undefinedimpl_1128 {
() => {
// Module: crate::types::maybe_undefined
// Provides: {"impl_1128"}
// Dependencies: {}
impl < T : Serialize > Serialize for MaybeUndefined < T > { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { match self { MaybeUndefined :: Value (value) => value . serialize (serializer) , _ => serializer . serialize_none () , } } }
};
}

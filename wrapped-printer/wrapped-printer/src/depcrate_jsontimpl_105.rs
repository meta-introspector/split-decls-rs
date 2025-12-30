// Generated macro for impl_105 (impl)
macro_rules! Depcrate_jsontimpl_105 {
() => {
// Module: crate::jsont
// Provides: {"impl_105"}
// Dependencies: {}
impl < 'a > serde :: Serialize for SubMatch < 'a > { fn serialize < S : serde :: Serializer > (& self , s : S ,) -> Result < S :: Ok , S :: Error > { use serde :: ser :: SerializeStruct ; let mut state = s . serialize_struct ("SubMatch" , 3) ? ; state . serialize_field ("match" , & Data :: from_bytes (self . m)) ? ; if let Some (r) = self . replacement { state . serialize_field ("replacement" , & Data :: from_bytes (r)) ? ; } state . serialize_field ("start" , & self . start) ? ; state . serialize_field ("end" , & self . end) ? ; state . end () } }
};
}

// Generated macro for impl_97 (impl)
macro_rules! Depcrate_jsontimpl_97 {
() => {
// Module: crate::jsont
// Provides: {"impl_97"}
// Dependencies: {}
impl < 'a > serde :: Serialize for Begin < 'a > { fn serialize < S : serde :: Serializer > (& self , s : S ,) -> Result < S :: Ok , S :: Error > { use serde :: ser :: SerializeStruct ; let mut state = s . serialize_struct ("Begin" , 1) ? ; state . serialize_field ("path" , & self . path . map (Data :: from_path)) ? ; state . end () } }
};
}

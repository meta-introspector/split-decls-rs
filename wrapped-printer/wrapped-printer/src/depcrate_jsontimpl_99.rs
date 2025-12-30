// Generated macro for impl_99 (impl)
macro_rules! Depcrate_jsontimpl_99 {
() => {
// Module: crate::jsont
// Provides: {"impl_99"}
// Dependencies: {}
impl < 'a > serde :: Serialize for End < 'a > { fn serialize < S : serde :: Serializer > (& self , s : S ,) -> Result < S :: Ok , S :: Error > { use serde :: ser :: SerializeStruct ; let mut state = s . serialize_struct ("End" , 3) ? ; state . serialize_field ("path" , & self . path . map (Data :: from_path)) ? ; state . serialize_field ("binary_offset" , & self . binary_offset) ? ; state . serialize_field ("stats" , & self . stats) ? ; state . end () } }
};
}

// Generated macro for impl_103 (impl)
macro_rules! Depcrate_jsontimpl_103 {
() => {
// Module: crate::jsont
// Provides: {"impl_103"}
// Dependencies: {}
impl < 'a > serde :: Serialize for Context < 'a > { fn serialize < S : serde :: Serializer > (& self , s : S ,) -> Result < S :: Ok , S :: Error > { use serde :: ser :: SerializeStruct ; let mut state = s . serialize_struct ("Context" , 5) ? ; state . serialize_field ("path" , & self . path . map (Data :: from_path)) ? ; state . serialize_field ("lines" , & Data :: from_bytes (self . lines)) ? ; state . serialize_field ("line_number" , & self . line_number) ? ; state . serialize_field ("absolute_offset" , & self . absolute_offset) ? ; state . serialize_field ("submatches" , & self . submatches) ? ; state . end () } }
};
}

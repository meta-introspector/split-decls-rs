// Generated macro for impl_240 (impl)
macro_rules! Depcrate_frameimpl_240 {
() => {
// Module: crate::frame
// Provides: {"impl_240"}
// Dependencies: {}
impl Serialize for EnrichedHeaders { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = s . serialize_struct ("enriched_headers" , 2) ? ; state . serialize_field ("header_block_len" , & self . header_block . len ()) ? ; let x : Vec < SerializableHeader > = self . headers . iter () . map (SerializableHeader) . collect () ; state . serialize_field ("headers" , & x) ? ; state . end () } }
};
}

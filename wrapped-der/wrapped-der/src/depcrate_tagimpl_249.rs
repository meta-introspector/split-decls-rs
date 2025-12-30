// Generated macro for impl_249 (impl)
macro_rules! Depcrate_tagimpl_249 {
() => {
// Module: crate::tag
// Provides: {"impl_249"}
// Dependencies: {}
impl < 'a > Decode < 'a > for Tag { type Error = Error ; fn decode < R : Reader < 'a > > (reader : & mut R) -> Result < Self > { Self :: decode_with_constructed_bit (reader) . map (| (tag , _) | tag) } }
};
}

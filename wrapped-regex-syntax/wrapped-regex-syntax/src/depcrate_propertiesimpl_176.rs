// Generated macro for impl_176 (impl)
macro_rules! Depcrate_propertiesimpl_176 {
() => {
// Module: crate::properties
// Provides: {"impl_176"}
// Dependencies: {}
impl Arbitrary for ByteRange { fn arbitrary < G : Gen > (g : & mut G) -> ByteRange { ByteRange :: new (g . gen_range (97 , 123) , g . gen_range (97 , 123)) } fn shrink (& self) -> Box < Iterator < Item = ByteRange > > { Box :: new ((self . start , self . end) . shrink () . map (| (s , e) | ByteRange :: new (s , e))) } }
};
}

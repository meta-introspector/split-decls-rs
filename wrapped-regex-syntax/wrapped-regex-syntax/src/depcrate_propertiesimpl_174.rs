// Generated macro for impl_174 (impl)
macro_rules! Depcrate_propertiesimpl_174 {
() => {
// Module: crate::properties
// Provides: {"impl_174"}
// Dependencies: {}
impl Arbitrary for ClassRange { fn arbitrary < G : Gen > (g : & mut G) -> ClassRange { use std :: char :: from_u32 ; ClassRange :: new (from_u32 (g . gen_range (97 , 123)) . unwrap () , from_u32 (g . gen_range (97 , 123)) . unwrap () ,) } fn shrink (& self) -> Box < Iterator < Item = ClassRange > > { Box :: new ((self . start , self . end) . shrink () . map (| (s , e) | ClassRange :: new (s , e))) } }
};
}

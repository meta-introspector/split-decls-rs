// Generated macro for impl_164 (impl)
macro_rules! Depcrate_propertiesimpl_164 {
() => {
// Module: crate::properties
// Provides: {"impl_164"}
// Dependencies: {}
impl Arbitrary for SmallAscii { fn arbitrary < G : Gen > (g : & mut G) -> SmallAscii { use std :: char :: from_u32 ; let size = g . gen_range (1 , 5) ; SmallAscii ((0 .. size) . map (| _ | from_u32 (g . gen_range (97 , 123)) . unwrap ()) . collect ()) } fn shrink (& self) -> Box < Iterator < Item = SmallAscii > > { Box :: new (self . 0 . shrink () . map (SmallAscii)) } }
};
}

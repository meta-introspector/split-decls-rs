// Generated macro for impl_71 (impl)
macro_rules! Depcrate_foreign_core_charimpl_71 {
() => {
// Module: crate::foreign::core::char
// Provides: {"impl_71"}
// Dependencies: {}
# [doc = " Returns '\\0', not an error, if this `Unstructured` [is empty][Unstructured::is_empty]."] impl < 'a > Arbitrary < 'a > for char { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { const CHAR_END : u32 = 0x11_0000 ; const SURROGATES_START : u32 = 0xD800 ; let mut c = < u32 as Arbitrary < 'a > > :: arbitrary (u) ? % CHAR_END ; if let Some (c) = char :: from_u32 (c) { Ok (c) } else { c -= SURROGATES_START ; Ok (char :: from_u32 (c) . expect ("Generated character should be valid! This is a bug in arbitrary-rs")) } } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < u32 as Arbitrary < 'a > > :: size_hint (depth) } }
};
}

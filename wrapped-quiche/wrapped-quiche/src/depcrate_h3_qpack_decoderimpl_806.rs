// Generated macro for impl_806 (impl)
macro_rules! Depcrate_h3_qpack_decoderimpl_806 {
() => {
// Module: crate::h3::qpack::decoder
// Provides: {"impl_806"}
// Dependencies: {}
impl Representation { pub fn from_byte (b : u8) -> Representation { if b & INDEXED == INDEXED { return Representation :: Indexed ; } if b & LITERAL_WITH_NAME_REF == LITERAL_WITH_NAME_REF { return Representation :: LiteralWithNameRef ; } if b & LITERAL == LITERAL { return Representation :: Literal ; } if b & INDEXED_WITH_POST_BASE == INDEXED_WITH_POST_BASE { return Representation :: IndexedWithPostBase ; } Representation :: LiteralWithPostBase } }
};
}

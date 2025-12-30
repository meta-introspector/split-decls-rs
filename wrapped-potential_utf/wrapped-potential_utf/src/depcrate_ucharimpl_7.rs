// Generated macro for impl_7 (impl)
macro_rules! Depcrate_ucharimpl_7 {
() => {
// Module: crate::uchar
// Provides: {"impl_7"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `zerovec` Cargo feature"] # [cfg (feature = "zerovec")] impl zerovec :: ule :: AsULE for PotentialCodePoint { type ULE = zerovec :: ule :: RawBytesULE < 3 > ; # [inline] fn to_unaligned (self) -> Self :: ULE { zerovec :: ule :: RawBytesULE (self . 0) } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { Self (unaligned . 0) } }
};
}

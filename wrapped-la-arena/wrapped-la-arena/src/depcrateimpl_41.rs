// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl < T > Idx < T > { # [doc = " Creates a new index from a [`RawIdx`]."] pub const fn from_raw (raw : RawIdx) -> Self { Idx { raw , _ty : PhantomData } } # [doc = " Converts this index into the underlying [`RawIdx`]."] pub const fn into_raw (self) -> RawIdx { self . raw } }
};
}

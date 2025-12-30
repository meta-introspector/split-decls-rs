// Generated macro for impl_285 (impl)
macro_rules! Depcrate_colorimpl_285 {
() => {
// Module: crate::color
// Provides: {"impl_285"}
// Dependencies: {}
impl FromPrimitive < u8 > for u16 { fn from_primitive (c8 : u8) -> Self { let x = c8 . to_u64 () . unwrap () ; NumCast :: from ((x << 8) | x) . unwrap () } }
};
}

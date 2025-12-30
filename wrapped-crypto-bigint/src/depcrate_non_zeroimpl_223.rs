// Generated macro for impl_223 (impl)
macro_rules! Depcrate_non_zeroimpl_223 {
() => {
// Module: crate::non_zero
// Provides: {"impl_223"}
// Dependencies: {}
impl < T > NonZero < T > where T : Bounded + ? Sized , { # [doc = " Total size of the represented integer in bits."] pub const BITS : u32 = T :: BITS ; # [doc = " Total size of the represented integer in bytes."] pub const BYTES : usize = T :: BYTES ; }
};
}

// Generated macro for impl_159 (impl)
macro_rules! Depcrate_bitsimpl_159 {
() => {
// Module: crate::bits
// Provides: {"impl_159"}
// Dependencies: {}
impl BitLength < usize > { # [cfg (feature = "alloc")] # [inline] pub (crate) fn half_rounded_up (& self) -> Self { let round_up = self . 0 & 1 ; Self ((self . 0 / 2) + round_up) } # [doc = " The bit length, rounded up to a whole number of bytes."] # [inline] pub const fn as_usize_bytes_rounded_up (& self) -> usize { let round_up = ((self . 0 >> 2) | (self . 0 >> 1) | self . 0) & 1 ; (self . 0 / 8) + round_up } # [cfg (feature = "alloc")] # [inline] pub (crate) fn try_sub_1 (self) -> Result < Self , crate :: error :: Unspecified > { self . checked_sub (Self (1)) . ok_or (crate :: error :: Unspecified) } # [cfg (feature = "alloc")] pub (crate) fn checked_sub (self , rhs : Self) -> Option < Self > { self . 0 . checked_sub (rhs . 0) . map (Self) } }
};
}

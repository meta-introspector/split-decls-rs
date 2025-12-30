// Generated macro for impl_501 (impl)
macro_rules! Depcrate_window5impl_501 {
() => {
// Module: crate::window5
// Provides: {"impl_501"}
// Dependencies: {}
impl LeakyWindow5 { pub const _0 : Self = Self (0) ; pub const _1 : Self = Self (1) ; # [allow (dead_code)] const MAX : Self = Self (31) ; # [cfg (target_arch = "x86_64")] pub fn checked_double (self) -> Option < Self > { if self . 0 >= 16 { return None ; } Some (Self (self . 0 * 2)) } # [cfg (target_arch = "x86_64")] pub fn checked_pred (self) -> Option < Self > { self . 0 . checked_sub (1) . map (Self) } # [allow (dead_code)] # [inline] pub fn leak_usize (self) -> usize { # [allow (clippy :: cast_possible_truncation)] const _LOSSLESS_CONVERSION : () = assert ! (LeakyWindow5 :: MAX . 0 as usize == 31) ; usize :: try_from (self . 0) . unwrap_or_else (| _ | unreachable ! ()) } # [cfg (target_arch = "x86_64")] pub fn range () -> impl Iterator < Item = Self > { (0 ..= (Self :: MAX . 0)) . map (Self) } }
};
}

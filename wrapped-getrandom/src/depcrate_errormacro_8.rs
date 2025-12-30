// Generated macro for macro_8 (macro)
macro_rules! Depcrate_errormacro_8 {
() => {
// Module: crate::error
// Provides: {"macro_8"}
// Dependencies: {}
cfg_if :: cfg_if ! (if # [cfg (target_os = "uefi")] { # [doc = " Raw error code."] # [doc = ""] # [doc = " This alias mirrors unstable [`std::io::RawOsError`]."] # [doc = ""] # [doc = " [`std::io::RawOsError`]: https://doc.rust-lang.org/std/io/type.RawOsError.html"] pub type RawOsError = usize ; type NonZeroRawOsError = core :: num :: NonZeroUsize ; const UEFI_ERROR_FLAG : RawOsError = 1 << (RawOsError :: BITS - 1) ; } else { # [doc = " Raw error code."] # [doc = ""] # [doc = " This alias mirrors unstable [`std::io::RawOsError`]."] # [doc = ""] # [doc = " [`std::io::RawOsError`]: https://doc.rust-lang.org/std/io/type.RawOsError.html"] pub type RawOsError = i32 ; type NonZeroRawOsError = core :: num :: NonZeroI32 ; }) ;
};
}

// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl RestoreState { # [doc = " Create an invalid, dummy  `RestoreState`."] # [doc = ""] # [doc = " This can be useful to avoid `Option` when storing a `RestoreState` in a"] # [doc = " struct field, or a `static`."] # [doc = ""] # [doc = " Note that due to the safety contract of [`acquire`]/[`release`], you must not pass"] # [doc = " a `RestoreState` obtained from this method to [`release`]."] pub const fn invalid () -> Self { # [cfg (not (any (feature = "restore-state-bool" , feature = "restore-state-u8" , feature = "restore-state-u16" , feature = "restore-state-u32" , feature = "restore-state-u64" , feature = "restore-state-usize")))] return Self (()) ; # [cfg (feature = "restore-state-bool")] return Self (false) ; # [cfg (feature = "restore-state-u8")] return Self (0) ; # [cfg (feature = "restore-state-u16")] return Self (0) ; # [cfg (feature = "restore-state-u32")] return Self (0) ; # [cfg (feature = "restore-state-u64")] return Self (0) ; # [cfg (feature = "restore-state-usize")] return Self (0) ; } }
};
}

// Generated macro for delegate_size (macro)
macro_rules! Depcrate_utilsdelegate_size {
() => {
// Module: crate::utils
// Provides: {"delegate_size"}
// Dependencies: {}
# [allow (unused_macros)] macro_rules ! delegate_size { ($ delegate : ident) => { # [cfg (target_pointer_width = "16")] $ delegate ! (isize , u16) ; # [cfg (target_pointer_width = "16")] $ delegate ! (usize , u16) ; # [cfg (target_pointer_width = "32")] $ delegate ! (isize , u32) ; # [cfg (target_pointer_width = "32")] $ delegate ! (usize , u32) ; # [cfg (target_pointer_width = "64")] $ delegate ! (isize , u64) ; # [cfg (target_pointer_width = "64")] $ delegate ! (usize , u64) ; # [cfg (target_pointer_width = "128")] $ delegate ! (isize , u128) ; # [cfg (target_pointer_width = "128")] $ delegate ! (usize , u128) ; } ; }
};
}

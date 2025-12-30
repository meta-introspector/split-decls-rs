// Generated macro for EXTERN (macro)
macro_rules! Depcrate_macrosEXTERN {
() => {
// Module: crate::macros
// Provides: {"EXTERN"}
// Dependencies: {}
# [macro_export] macro_rules ! EXTERN { (extern $ c : tt { $ (fn $ n : ident ($ ($ p : tt $ (: $ t : ty) ?) ,* $ (,) ?) $ (-> $ r : ty) ?;) + }) => { # [cfg_attr (all (target_env = "msvc" , feature = "user") , link (name = "ntdll"))] # [cfg_attr (all (target_env = "msvc" , feature = "kernel") , link (name = "ntoskrnl"))] extern $ c { $ (pub fn $ n ($ ($ p $ (: $ t) ?) ,*) $ (-> $ r) ?;) + } $ (# [cfg (feature = "func-types")] pub type $ n = unsafe extern $ c fn ($ ($ p $ (: $ t) ?) ,*) $ (-> $ r) ?;) + } ; (extern $ c : tt { $ (static mut $ n : ident : $ t : ty ;) + }) => { # [cfg_attr (all (target_env = "msvc" , feature = "user") , link (name = "ntdll"))] extern $ c { $ (pub static mut $ n : $ t ;) + } } ; }
};
}

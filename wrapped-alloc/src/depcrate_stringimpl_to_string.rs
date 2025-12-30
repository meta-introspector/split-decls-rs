// Generated macro for impl_to_string (macro)
macro_rules! Depcrate_stringimpl_to_string {
() => {
// Module: crate::string
// Provides: {"impl_to_string"}
// Dependencies: {}
macro_rules ! impl_to_string { ($ ($ signed : ident , $ unsigned : ident ,) *) => { $ (# [cfg (not (no_global_oom_handling))] # [cfg (not (feature = "optimize_for_size"))] impl SpecToString for $ signed { # [inline] fn spec_to_string (& self) -> String { const SIZE : usize = $ signed :: MAX . ilog10 () as usize + 1 ; let mut buf = [core :: mem :: MaybeUninit ::< u8 >:: uninit () ; SIZE] ; let mut out ; if * self < 0 { out = String :: with_capacity (SIZE + 1) ; out . push ('-') ; } else { out = String :: with_capacity (SIZE) ; } unsafe { out . push_str (self . unsigned_abs () . _fmt (& mut buf)) ; } out } } # [cfg (not (no_global_oom_handling))] # [cfg (not (feature = "optimize_for_size"))] impl SpecToString for $ unsigned { # [inline] fn spec_to_string (& self) -> String { const SIZE : usize = $ unsigned :: MAX . ilog10 () as usize + 1 ; let mut buf = [core :: mem :: MaybeUninit ::< u8 >:: uninit () ; SIZE] ; unsafe { self . _fmt (& mut buf) . to_string () } } }) * } }
};
}

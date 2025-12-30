// Generated macro for impl_default_bit_opts (macro)
macro_rules! Depcrate_utilsimpl_default_bit_opts {
() => {
// Module: crate::utils
// Provides: {"impl_default_bit_opts"}
// Dependencies: {}
macro_rules ! impl_default_bit_opts { ($ atomic_type : ident , $ int_type : ty) => { impl $ atomic_type { # [inline] # [cfg_attr (miri , track_caller)] pub (crate) fn bit_set (& self , bit : u32 , order : Ordering) -> bool { let mask = <$ int_type >:: wrapping_shl (1 , bit) ; self . fetch_or (mask , order) & mask != 0 } # [inline] # [cfg_attr (miri , track_caller)] pub (crate) fn bit_clear (& self , bit : u32 , order : Ordering) -> bool { let mask = <$ int_type >:: wrapping_shl (1 , bit) ; self . fetch_and (! mask , order) & mask != 0 } # [inline] # [cfg_attr (miri , track_caller)] pub (crate) fn bit_toggle (& self , bit : u32 , order : Ordering) -> bool { let mask = <$ int_type >:: wrapping_shl (1 , bit) ; self . fetch_xor (mask , order) & mask != 0 } } } ; }
};
}

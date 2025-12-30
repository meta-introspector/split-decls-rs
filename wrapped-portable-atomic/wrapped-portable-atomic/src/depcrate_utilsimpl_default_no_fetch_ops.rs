// Generated macro for impl_default_no_fetch_ops (macro)
macro_rules! Depcrate_utilsimpl_default_no_fetch_ops {
() => {
// Module: crate::utils
// Provides: {"impl_default_no_fetch_ops"}
// Dependencies: {}
macro_rules ! impl_default_no_fetch_ops { ($ atomic_type : ident , bool) => { impl $ atomic_type { # [inline] # [cfg_attr (miri , track_caller)] pub (crate) fn and (& self , val : bool , order : Ordering) { self . fetch_and (val , order) ; } # [inline] # [cfg_attr (miri , track_caller)] pub (crate) fn or (& self , val : bool , order : Ordering) { self . fetch_or (val , order) ; } # [inline] # [cfg_attr (miri , track_caller)] pub (crate) fn xor (& self , val : bool , order : Ordering) { self . fetch_xor (val , order) ; } } } ; ($ atomic_type : ident , $ int_type : ty) => { impl $ atomic_type { # [inline] # [cfg_attr (miri , track_caller)] pub (crate) fn add (& self , val : $ int_type , order : Ordering) { self . fetch_add (val , order) ; } # [inline] # [cfg_attr (miri , track_caller)] pub (crate) fn sub (& self , val : $ int_type , order : Ordering) { self . fetch_sub (val , order) ; } # [inline] # [cfg_attr (miri , track_caller)] pub (crate) fn and (& self , val : $ int_type , order : Ordering) { self . fetch_and (val , order) ; } # [inline] # [cfg_attr (miri , track_caller)] pub (crate) fn or (& self , val : $ int_type , order : Ordering) { self . fetch_or (val , order) ; } # [inline] # [cfg_attr (miri , track_caller)] pub (crate) fn xor (& self , val : $ int_type , order : Ordering) { self . fetch_xor (val , order) ; } } } ; }
};
}

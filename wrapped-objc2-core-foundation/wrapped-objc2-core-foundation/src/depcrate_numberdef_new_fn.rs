// Generated macro for def_new_fn (macro)
macro_rules! Depcrate_numberdef_new_fn {
() => {
// Module: crate::number
// Provides: {"def_new_fn"}
// Dependencies: {}
macro_rules ! def_new_fn { { $ ($ (# [$ ($ m : meta) *]) * ($ fn_name : ident ($ fn_inp : ty) ; $ type : ident) ,) * } => { $ ($ (# [$ ($ m) *]) * # [inline] pub fn $ fn_name (val : $ fn_inp) -> CFRetained < Self > { let ptr : * const $ fn_inp = & val ; unsafe { Self :: new (None , CFNumberType ::$ type , ptr . cast ()) . expect ("failed creating CFNumber") } }) * } }
};
}

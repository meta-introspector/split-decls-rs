// Generated macro for def_get_fn (macro)
macro_rules! Depcrate_numberdef_get_fn {
() => {
// Module: crate::number
// Provides: {"def_get_fn"}
// Dependencies: {}
macro_rules ! def_get_fn { { $ ($ (# [$ ($ m : meta) *]) * ($ fn_name : ident -> $ fn_ret : ty ; $ type : ident) ,) * } => { $ ($ (# [$ ($ m) *]) * # [inline] pub fn $ fn_name (& self) -> Option <$ fn_ret > { let mut value : $ fn_ret = <$ fn_ret >:: default () ; let ptr : * mut $ fn_ret = & mut value ; let ret = unsafe { self . value (CFNumberType ::$ type , ptr . cast ()) } ; if ret { Some (value) } else { None } }) * } }
};
}

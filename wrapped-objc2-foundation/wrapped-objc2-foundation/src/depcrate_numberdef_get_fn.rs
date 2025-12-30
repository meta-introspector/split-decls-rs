// Generated macro for def_get_fn (macro)
macro_rules! Depcrate_numberdef_get_fn {
() => {
// Module: crate::number
// Provides: {"def_get_fn"}
// Dependencies: {}
macro_rules ! def_get_fn { { $ ($ (# [$ ($ m : meta) *]) * ($ fn_name : ident -> $ fn_ret : ty ; $ method_name : ident) ,) * } => { $ ($ (# [$ ($ m) *]) * pub fn $ fn_name (& self) -> $ fn_ret { self .$ method_name () as _ }) * } }
};
}

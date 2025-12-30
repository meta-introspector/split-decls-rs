// Generated macro for def_new_fn (macro)
macro_rules! Depcrate_numberdef_new_fn {
() => {
// Module: crate::number
// Provides: {"def_new_fn"}
// Dependencies: {}
macro_rules ! def_new_fn { { $ ($ (# [$ ($ m : meta) *]) * ($ fn_name : ident ($ fn_inp : ty) ; $ method_name : ident) ,) * } => { $ ($ (# [$ ($ m) *]) * pub fn $ fn_name (val : $ fn_inp) -> Retained < Self > { Self ::$ method_name (val as _) }) * } }
};
}

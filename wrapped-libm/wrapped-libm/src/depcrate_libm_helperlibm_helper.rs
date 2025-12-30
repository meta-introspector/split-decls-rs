// Generated macro for libm_helper (macro)
macro_rules! Depcrate_libm_helperlibm_helper {
() => {
// Module: crate::libm_helper
// Provides: {"libm_helper"}
// Dependencies: {}
macro_rules ! libm_helper { ($ t : ident , funcs : $ funcs : tt) => { impl Libm <$ t > { #! [allow (unused_parens)] libm_helper ! { $ funcs } } } ; ({ $ ($ func : tt ;) * }) => { $ (libm_helper ! { $ func }) * } ; ((fn $ func : ident ($ ($ arg : ident : $ arg_typ : ty) ,*) -> ($ ($ ret_typ : ty) ,*) ; => $ libm_fn : ident)) => { # [inline (always)] pub fn $ func ($ ($ arg : $ arg_typ) ,*) -> ($ ($ ret_typ) ,*) { $ libm_fn ($ ($ arg) ,*) } } ; }
};
}

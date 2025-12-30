// Generated macro for export_out_param (macro)
macro_rules! Depcrate_syscall_mathexport_out_param {
() => {
// Module: crate::syscall::math
// Provides: {"export_out_param"}
// Dependencies: {}
macro_rules ! export_out_param { ($ (fn $ fn : ident ($ ($ arg : ident : $ argty : ty) ,+; $ out : ident : $ outty : ty) -> $ retty : ty ;) +) => { $ (# [no_mangle] pub extern "C" fn $ fn ($ ($ arg : $ argty) ,+, $ out : $ outty) -> $ retty { let (ret , out) = :: libm ::$ fn ($ ($ arg) ,+) ; *$ out = out ; ret }) + } ; }
};
}

// Generated macro for export (macro)
macro_rules! Depcrate_syscall_mathexport {
() => {
// Module: crate::syscall::math
// Provides: {"export"}
// Dependencies: {}
macro_rules ! export { ($ (fn $ fn : ident ($ ($ arg : ident : $ argty : ty) ,+) -> $ retty : ty ;) +) => { $ (# [no_mangle] pub extern "C" fn $ fn ($ ($ arg : $ argty) ,+) -> $ retty { :: libm ::$ fn ($ ($ arg) ,+) }) + } ; }
};
}

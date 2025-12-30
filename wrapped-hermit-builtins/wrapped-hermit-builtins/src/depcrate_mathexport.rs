// Generated macro for export (macro)
macro_rules! Depcrate_mathexport {
() => {
// Module: crate::math
// Provides: {"export"}
// Dependencies: {}
macro_rules ! export { ($ (fn $ fn : ident ($ ($ arg : ident : $ argty : ty) ,+) -> $ retty : ty ;) +) => { $ (# [linkage = "weak_odr"] # [unsafe (no_mangle)] pub extern "C" fn $ fn ($ ($ arg : $ argty) ,+) -> $ retty { :: libm ::$ fn ($ ($ arg) ,+) }) + } ; }
};
}

// Generated macro for i (macro)
macro_rules! Depcrate_libmi {
() => {
// Module: crate::libm
// Provides: {"i"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Safe if `index < array.len()`."] macro_rules ! i { ($ array : ident , $ index : expr) => { unsafe { *$ array . get_unchecked ($ index) } } ; }
};
}

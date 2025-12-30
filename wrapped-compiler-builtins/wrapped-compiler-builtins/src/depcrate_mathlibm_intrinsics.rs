// Generated macro for libm_intrinsics (macro)
macro_rules! Depcrate_mathlibm_intrinsics {
() => {
// Module: crate::math
// Provides: {"libm_intrinsics"}
// Dependencies: {}
macro_rules ! libm_intrinsics { ($ (fn $ fun : ident ($ ($ iid : ident : $ ity : ty) ,+) -> $ oty : ty ;) +) => { intrinsics ! { $ (pub extern "C" fn $ fun ($ ($ iid : $ ity) ,+) -> $ oty { $ crate :: math :: libm_math ::$ fun ($ ($ iid) ,+) }) + } } }
};
}

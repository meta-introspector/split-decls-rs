// Generated macro for fn_alias (macro)
macro_rules! Depcrate_utilsfn_alias {
() => {
// Module: crate::utils
// Provides: {"fn_alias"}
// Dependencies: {}
# [allow (unused_macros)] # [cfg (not (portable_atomic_no_outline_atomics))] # [cfg (any (target_arch = "aarch64" , target_arch = "arm" , target_arch = "arm64ec" , target_arch = "powerpc64" , target_arch = "riscv32" , target_arch = "riscv64" , all (target_arch = "x86_64" , not (any (target_env = "sgx" , miri))) ,))] macro_rules ! fn_alias { ($ (# [$ ($ fn_attr : tt) *]) * $ vis : vis unsafe fn ($ ($ arg_pat : ident : $ arg_ty : ty) ,*) $ (-> $ ret_ty : ty) ?; $ (# [$ ($ alias_attr : tt) *]) * $ new : ident = $ from : ident ($ ($ last_args : tt) *) ; $ ($ rest : tt) *) => { $ (# [$ ($ fn_attr) *]) * $ (# [$ ($ alias_attr) *]) * $ vis unsafe fn $ new ($ ($ arg_pat : $ arg_ty) ,*) $ (-> $ ret_ty) ? { unsafe { $ from ($ ($ arg_pat ,) * $ ($ last_args) *) } } fn_alias ! { $ (# [$ ($ fn_attr) *]) * $ vis unsafe fn ($ ($ arg_pat : $ arg_ty) ,*) $ (-> $ ret_ty) ?; $ ($ rest) * } } ; ($ (# [$ ($ attr : tt) *]) * $ vis : vis unsafe fn ($ ($ arg_pat : ident : $ arg_ty : ty) ,*) $ (-> $ ret_ty : ty) ?;) => { } }
};
}

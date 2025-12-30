// Generated macro for ifunc (macro)
macro_rules! Depcrate_utilsifunc {
() => {
// Module: crate::utils
// Provides: {"ifunc"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " - the caller must uphold the safety contract for the function returned by $detect_body."] # [doc = " - the memory pointed by the function pointer returned by $detect_body must be visible from any threads."] # [doc = ""] # [doc = " The second requirement is always met if the function pointer is to the function definition."] # [doc = " (Currently, all uses of this macro in our code are in this case.)"] # [allow (unused_macros)] # [cfg (not (portable_atomic_no_outline_atomics))] # [cfg (any (target_arch = "aarch64" , target_arch = "arm" , target_arch = "arm64ec" , target_arch = "powerpc64" , target_arch = "riscv32" , target_arch = "riscv64" , all (target_arch = "x86_64" , not (any (target_env = "sgx" , miri))) ,))] macro_rules ! ifunc { (unsafe fn ($ ($ arg_pat : ident : $ arg_ty : ty) ,*) $ (-> $ ret_ty : ty) ? { $ ($ detect_body : tt) * }) => { { type FnTy = unsafe fn ($ ($ arg_ty) ,*) $ (-> $ ret_ty) ?; static FUNC : core :: sync :: atomic :: AtomicPtr < () > = core :: sync :: atomic :: AtomicPtr :: new (detect as * mut ()) ; # [cold] unsafe fn detect ($ ($ arg_pat : $ arg_ty) ,*) $ (-> $ ret_ty) ? { let func : FnTy = { $ ($ detect_body) * } ; FUNC . store (func as * mut () , core :: sync :: atomic :: Ordering :: Relaxed) ; unsafe { func ($ ($ arg_pat) ,*) } } let func = { core :: mem :: transmute ::<* mut () , FnTy > (FUNC . load (core :: sync :: atomic :: Ordering :: Relaxed)) } ; func ($ ($ arg_pat) ,*) } } ; }
};
}

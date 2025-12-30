// Generated macro for test (module)
macro_rules! Depcrate_reg_contexttest {
() => {
// Module: crate::reg_context
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use std :: mem :: transmute ; use crate :: reg_context :: RegContext ; use crate :: stack :: Stack ; const MIN_STACK : usize = 1024 ; fn init_fn_impl (arg : usize , f : * mut usize) -> ! { let func : fn () = unsafe { transmute (f) } ; func () ; let ctx : & RegContext = unsafe { & * std :: ptr :: with_exposed_provenance (arg) } ; RegContext :: load (ctx) ; unreachable ! ("Should never comeback") ; } # [cfg (target_arch = "x86_64")] extern "sysv64" fn init_fn (arg : usize , f : * mut usize) -> ! { init_fn_impl (arg , f) } # [cfg (target_arch = "aarch64")] extern "C" fn init_fn (arg : usize , f : * mut usize) -> ! { init_fn_impl (arg , f) } # [cfg (target_arch = "loongarch64")] extern "C" fn init_fn (arg : usize , f : * mut usize) -> ! { init_fn_impl (arg , f) } # [cfg (target_arch = "riscv64")] extern "C" fn init_fn (arg : usize , f : * mut usize) -> ! { init_fn_impl (arg , f) } # [cfg (target_arch = "powerpc64")] extern "C" fn init_fn (arg : usize , f : * mut usize) -> ! { init_fn_impl (arg , f) } # [cfg (target_arch = "arm")] extern "aapcs" fn init_fn (arg : usize , f : * mut usize) -> ! { init_fn_impl (arg , f) } # [test] fn test_swap_context () { static mut VAL : bool = false ; let mut cur = RegContext :: empty () ; fn callback () { unsafe { VAL = true } ; } let stk = Stack :: new (MIN_STACK) ; let ctx = RegContext :: new (init_fn , & cur as * const _ as usize , callback as * mut usize , & stk ,) ; RegContext :: swap (& mut cur , & ctx) ; unsafe { assert ! (VAL) ; } } }
};
}

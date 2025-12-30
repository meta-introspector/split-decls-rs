// Generated macro for impl_41 (impl)
macro_rules! Depcrate_gen_implimpl_41 {
() => {
// Module: crate::gen_impl
// Provides: {"impl_41"}
// Dependencies: {}
impl < A : Any > Gn < A > { # [doc = " create a new generator with default stack size"] # [allow (clippy :: new_ret_no_self)] # [deprecated (since = "0.6.18" , note = "please use `scope` version instead")] pub fn new < 'a , T : Any , F > (f : F) -> Generator < 'a , A , T > where F : FnOnce () -> T + Send + 'a , { Self :: new_opt (DEFAULT_STACK_SIZE , f) } # [doc = " create a new generator with specified stack size"] pub fn new_opt < 'a , T : Any , F > (size : usize , f : F) -> Generator < 'a , A , T > where F : FnOnce () -> T + Send + 'a , { let mut gen = GeneratorImpl :: < A , T > :: new (Stack :: new (size)) ; gen . init_context () ; gen . init_code (f) ; Generator { gen } } }
};
}

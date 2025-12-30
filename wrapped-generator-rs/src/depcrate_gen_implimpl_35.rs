// Generated macro for impl_35 (impl)
macro_rules! Depcrate_gen_implimpl_35 {
() => {
// Module: crate::gen_impl
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'a , A , T > LocalGenerator < 'a , A , T > { # [doc = " init a heap based generator with scoped closure"] pub fn scoped_init < F > (& mut self , f : F) where for < 'scope > F : FnOnce (Scope < 'scope , 'a , A , T >) -> T + 'a , T : 'a , A : 'a , { self . gen . scoped_init (f) ; } }
};
}

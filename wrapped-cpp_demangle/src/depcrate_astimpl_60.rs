// Generated macro for impl_60 (impl)
macro_rules! Depcrate_astimpl_60 {
() => {
// Module: crate::ast
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'ctx , 'a , W > AutoDemangleContextInnerBarrier < 'ctx , 'a , W > where W : 'a + DemangleWrite , 'a : 'ctx , { # [doc = " Set aside the current inner stack on the demangle context."] pub fn new (ctx : & 'ctx mut DemangleContext < 'a , W >) -> Self { let mut saved_inner = vec ! [] ; mem :: swap (& mut saved_inner , & mut ctx . inner) ; AutoDemangleContextInnerBarrier { ctx : ctx , saved_inner : saved_inner , } } }
};
}

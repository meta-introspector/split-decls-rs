// Generated macro for impl_61 (impl)
macro_rules! Depcrate_astimpl_61 {
() => {
// Module: crate::ast
// Provides: {"impl_61"}
// Dependencies: {}
impl < 'ctx , 'a , W > ops :: Deref for AutoDemangleContextInnerBarrier < 'ctx , 'a , W > where W : 'a + DemangleWrite , 'a : 'ctx , { type Target = DemangleContext < 'a , W > ; fn deref (& self) -> & Self :: Target { self . ctx } }
};
}

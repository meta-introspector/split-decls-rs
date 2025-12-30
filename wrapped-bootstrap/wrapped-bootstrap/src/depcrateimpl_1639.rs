// Generated macro for impl_1639 (impl)
macro_rules! Depcrateimpl_1639 {
() => {
// Module: crate
// Provides: {"impl_1639"}
// Dependencies: {}
impl Mode { pub fn is_tool (& self) -> bool { match self { Mode :: ToolBootstrap | Mode :: ToolRustcPrivate | Mode :: ToolStd | Mode :: ToolTarget => true , Mode :: Std | Mode :: Codegen | Mode :: Rustc => false , } } pub fn must_support_dlopen (& self) -> bool { match self { Mode :: Std | Mode :: Codegen => true , Mode :: ToolBootstrap | Mode :: ToolRustcPrivate | Mode :: ToolStd | Mode :: ToolTarget | Mode :: Rustc => false , } } }
};
}

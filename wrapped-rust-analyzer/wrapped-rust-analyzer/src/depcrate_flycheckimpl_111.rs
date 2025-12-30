// Generated macro for impl_111 (impl)
macro_rules! Depcrate_flycheckimpl_111 {
() => {
// Module: crate::flycheck
// Provides: {"impl_111"}
// Dependencies: {}
impl FlycheckConfig { pub (crate) fn invocation_strategy (& self) -> InvocationStrategy { match self { FlycheckConfig :: CargoCommand { .. } => InvocationStrategy :: PerWorkspace , FlycheckConfig :: CustomCommand { invocation_strategy , .. } => { invocation_strategy . clone () } } } }
};
}

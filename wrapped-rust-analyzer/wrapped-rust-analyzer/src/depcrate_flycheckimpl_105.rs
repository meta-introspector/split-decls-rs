// Generated macro for impl_105 (impl)
macro_rules! Depcrate_flycheckimpl_105 {
() => {
// Module: crate::flycheck
// Provides: {"impl_105"}
// Dependencies: {}
impl FlycheckConfig { pub (crate) fn invocation_strategy_once (& self) -> bool { match self { FlycheckConfig :: CargoCommand { .. } => false , FlycheckConfig :: CustomCommand { invocation_strategy , .. } => { * invocation_strategy == InvocationStrategy :: Once } } } }
};
}

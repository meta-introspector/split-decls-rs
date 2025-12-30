// Generated macro for impl_1430 (impl)
macro_rules! Depcrate_utils_execimpl_1430 {
() => {
// Module: crate::utils::exec
// Provides: {"impl_1430"}
// Dependencies: {}
impl OutputMode { pub fn captures (& self) -> bool { match self { OutputMode :: Print => false , OutputMode :: Capture => true , } } pub fn stdio (& self) -> Stdio { match self { OutputMode :: Print => Stdio :: inherit () , OutputMode :: Capture => Stdio :: piped () , } } }
};
}

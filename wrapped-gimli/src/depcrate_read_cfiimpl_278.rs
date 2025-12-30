// Generated macro for impl_278 (impl)
macro_rules! Depcrate_read_cfiimpl_278 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_278"}
// Dependencies: {}
impl < T : ReaderOffset > CfaRule < T > { fn is_default (& self) -> bool { match * self { CfaRule :: RegisterAndOffset { register , offset } => { register == Register (0) && offset == 0 } _ => false , } } }
};
}
